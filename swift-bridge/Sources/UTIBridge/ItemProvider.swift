import Dispatch
import Foundation
import UniformTypeIdentifiers

final class ItemProviderBox: NSObject {
    let inner: NSItemProvider

    init(_ provider: NSItemProvider) {
        self.inner = provider
    }
}

final class DataRepresentationLoadState: @unchecked Sendable {
    var result: Data?
    var error: Error?
}

final class FileRepresentationLoadState: @unchecked Sendable {
    var url: URL?
    var openInPlace = false
    var error: Error?
}

func makeProviderOpaque(_ provider: NSItemProvider) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(ItemProviderBox(provider)).toOpaque()
}

func unmanagedProviderBox(_ pointer: UnsafeMutableRawPointer) -> Unmanaged<ItemProviderBox> {
    let typed = pointer.assumingMemoryBound(to: ItemProviderBox.self)
    return Unmanaged<ItemProviderBox>.fromOpaque(UnsafeRawPointer(typed))
}

func unboxProvider(_ pointer: UnsafeMutableRawPointer) -> NSItemProvider {
    unmanagedProviderBox(pointer).takeUnretainedValue().inner
}

func visibility(from rawValue: Int64) -> NSItemProviderRepresentationVisibility {
    NSItemProviderRepresentationVisibility(rawValue: Int(rawValue)) ?? .all
}

public typealias ItemProviderDataAsyncCallback = @convention(c) (
    UnsafePointer<UInt8>?, Int, UnsafePointer<CChar>?, UnsafeMutableRawPointer
) -> Void

public typealias ItemProviderFileAsyncCallback = @convention(c) (
    UnsafePointer<CChar>?, Bool, UnsafePointer<CChar>?, UnsafeMutableRawPointer
) -> Void

@_cdecl("item_provider_release")
public func item_provider_release(_ pointer: UnsafeMutableRawPointer?) {
    guard let pointer else { return }
    unmanagedProviderBox(pointer).release()
}

@_cdecl("item_provider_retain")
public func item_provider_retain(_ pointer: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let pointer else { return nil }
    let box = unmanagedProviderBox(pointer).takeUnretainedValue()
    return Unmanaged.passRetained(box).toOpaque()
}

@_cdecl("item_provider_new")
public func item_provider_new() -> UnsafeMutableRawPointer? {
    makeProviderOpaque(NSItemProvider())
}

@_cdecl("item_provider_from_file_path")
public func item_provider_from_file_path(
    _ path: UnsafePointer<CChar>,
    _ contentType: UnsafeMutableRawPointer?,
    _ openInPlace: Bool,
    _ coordinated: Bool,
    _ visibilityRaw: Int64,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    errorOut?.pointee = nil
    let provider = NSItemProvider(
        contentsOf: URL(fileURLWithPath: String(cString: path)),
        contentType: contentType.map(unbox),
        openInPlace: openInPlace,
        coordinated: coordinated,
        visibility: visibility(from: visibilityRaw)
    )
    return makeProviderOpaque(provider)
}

@_cdecl("item_provider_register_data_representation")
public func item_provider_register_data_representation(
    _ provider: UnsafeMutableRawPointer,
    _ contentType: UnsafeMutableRawPointer,
    _ visibilityRaw: Int64,
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int
) {
    let data = len == 0 ? Data() : Data(bytes: bytes!, count: len)
    unboxProvider(provider).registerDataRepresentation(
        for: unbox(contentType),
        visibility: visibility(from: visibilityRaw)
    ) { completion in
        completion(data, nil)
        return nil
    }
}

@_cdecl("item_provider_register_file_representation")
public func item_provider_register_file_representation(
    _ provider: UnsafeMutableRawPointer,
    _ contentType: UnsafeMutableRawPointer,
    _ visibilityRaw: Int64,
    _ openInPlace: Bool,
    _ path: UnsafePointer<CChar>,
    _ coordinated: Bool
) {
    let fileURL = URL(fileURLWithPath: String(cString: path))
    unboxProvider(provider).registerFileRepresentation(
        for: unbox(contentType),
        visibility: visibility(from: visibilityRaw),
        openInPlace: openInPlace
    ) { completion in
        completion(fileURL, coordinated, nil)
        return nil
    }
}

@_cdecl("item_provider_registered_type_identifiers")
public func item_provider_registered_type_identifiers(
    _ provider: UnsafeMutableRawPointer
) -> UnsafeMutablePointer<CChar>? {
    ffiJoinedStrings(unboxProvider(provider).registeredContentTypes.map(\.identifier))
}

@_cdecl("item_provider_registered_type_identifiers_with_file_options")
public func item_provider_registered_type_identifiers_with_file_options(
    _ provider: UnsafeMutableRawPointer,
    _ openInPlace: Bool
) -> UnsafeMutablePointer<CChar>? {
    let contentTypes = openInPlace
        ? unboxProvider(provider).registeredContentTypesForOpenInPlace
        : unboxProvider(provider).registeredContentTypes
    return ffiJoinedStrings(contentTypes.map(\.identifier))
}

@_cdecl("item_provider_load_data_representation")
public func item_provider_load_data_representation(
    _ provider: UnsafeMutableRawPointer,
    _ contentType: UnsafeMutableRawPointer,
    _ outLen: UnsafeMutablePointer<Int>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<UInt8>? {
    errorOut?.pointee = nil
    outLen?.pointee = 0
    let semaphore = DispatchSemaphore(value: 0)
    let state = DataRepresentationLoadState()
    _ = unboxProvider(provider).loadDataRepresentation(for: unbox(contentType)) { data, error in
        state.result = data
        state.error = error
        semaphore.signal()
    }
    if semaphore.wait(timeout: .now() + 30) == .timedOut {
        errorOut?.pointee = ffiString("loadDataRepresentation timed out")
        return nil
    }
    if let resultError = state.error {
        errorOut?.pointee = ffiString(resultError.localizedDescription)
        return nil
    }
    outLen?.pointee = state.result?.count ?? 0
    return ffiData(state.result)
}

@_cdecl("item_provider_load_file_representation")
public func item_provider_load_file_representation(
    _ provider: UnsafeMutableRawPointer,
    _ contentType: UnsafeMutableRawPointer,
    _ openInPlace: Bool,
    _ outOpenInPlace: UnsafeMutablePointer<Bool>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    errorOut?.pointee = nil
    outOpenInPlace?.pointee = false
    let semaphore = DispatchSemaphore(value: 0)
    let state = FileRepresentationLoadState()
    _ = unboxProvider(provider).loadFileRepresentation(
        for: unbox(contentType),
        openInPlace: openInPlace
    ) { url, isInPlace, error in
        state.url = url
        state.openInPlace = isInPlace
        state.error = error
        semaphore.signal()
    }
    if semaphore.wait(timeout: .now() + 30) == .timedOut {
        errorOut?.pointee = ffiString("loadFileRepresentation timed out")
        return nil
    }
    if let resultError = state.error {
        errorOut?.pointee = ffiString(resultError.localizedDescription)
        return nil
    }
    outOpenInPlace?.pointee = state.openInPlace
    return ffiString(state.url?.path)
}

@_cdecl("item_provider_load_data_representation_async")
public func item_provider_load_data_representation_async(
    _ provider: UnsafeMutableRawPointer?,
    _ contentType: UnsafeMutableRawPointer?,
    _ cb: @escaping ItemProviderDataAsyncCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    guard let provider else {
        "missing NSItemProvider".withCString { cb(nil, 0, $0, ctx) }
        return
    }
    guard let contentType else {
        "missing UTType".withCString { cb(nil, 0, $0, ctx) }
        return
    }

    let unmanaged = unmanagedProviderBox(provider)
    _ = unmanaged.retain()
    let itemProvider = unmanaged.takeUnretainedValue().inner
    let context = UInt(bitPattern: ctx)

    _ = itemProvider.loadDataRepresentation(for: unbox(contentType)) { data, error in
        defer { unmanaged.release() }
        let callbackContext = UnsafeMutableRawPointer(bitPattern: context)!
        if let error {
            error.localizedDescription.withCString { cb(nil, 0, $0, callbackContext) }
            return
        }
        guard let data else {
            "loadDataRepresentation returned no data".withCString {
                cb(nil, 0, $0, callbackContext)
            }
            return
        }
        data.withUnsafeBytes { rawBuffer in
            let bytes = rawBuffer.bindMemory(to: UInt8.self)
            cb(bytes.baseAddress, data.count, nil, callbackContext)
        }
    }
}

@_cdecl("item_provider_load_file_representation_async")
public func item_provider_load_file_representation_async(
    _ provider: UnsafeMutableRawPointer?,
    _ contentType: UnsafeMutableRawPointer?,
    _ openInPlace: Bool,
    _ cb: @escaping ItemProviderFileAsyncCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    guard let provider else {
        "missing NSItemProvider".withCString { cb(nil, false, $0, ctx) }
        return
    }
    guard let contentType else {
        "missing UTType".withCString { cb(nil, false, $0, ctx) }
        return
    }

    let unmanaged = unmanagedProviderBox(provider)
    _ = unmanaged.retain()
    let itemProvider = unmanaged.takeUnretainedValue().inner
    let context = UInt(bitPattern: ctx)

    _ = itemProvider.loadFileRepresentation(
        for: unbox(contentType),
        openInPlace: openInPlace
    ) { url, isInPlace, error in
        defer { unmanaged.release() }
        let callbackContext = UnsafeMutableRawPointer(bitPattern: context)!
        if let error {
            error.localizedDescription.withCString { cb(nil, false, $0, callbackContext) }
            return
        }
        guard let url else {
            "loadFileRepresentation returned no file URL".withCString {
                cb(nil, false, $0, callbackContext)
            }
            return
        }
        url.path.withCString { cb($0, isInPlace, nil, callbackContext) }
    }
}
