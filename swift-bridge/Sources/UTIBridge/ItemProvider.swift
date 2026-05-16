import Dispatch
import Foundation
import UniformTypeIdentifiers

final class ItemProviderBox: NSObject {
    let inner: NSItemProvider

    init(_ provider: NSItemProvider) {
        self.inner = provider
    }
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
    var result: Data?
    var resultError: Error?
    _ = unboxProvider(provider).loadDataRepresentation(for: unbox(contentType)) { data, error in
        result = data
        resultError = error
        semaphore.signal()
    }
    if semaphore.wait(timeout: .now() + 30) == .timedOut {
        errorOut?.pointee = ffiString("loadDataRepresentation timed out")
        return nil
    }
    if let resultError {
        errorOut?.pointee = ffiString(resultError.localizedDescription)
        return nil
    }
    outLen?.pointee = result?.count ?? 0
    return ffiData(result)
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
    var resultURL: URL?
    var actualOpenInPlace = false
    var resultError: Error?
    _ = unboxProvider(provider).loadFileRepresentation(
        for: unbox(contentType),
        openInPlace: openInPlace
    ) { url, isInPlace, error in
        resultURL = url
        actualOpenInPlace = isInPlace
        resultError = error
        semaphore.signal()
    }
    if semaphore.wait(timeout: .now() + 30) == .timedOut {
        errorOut?.pointee = ffiString("loadFileRepresentation timed out")
        return nil
    }
    if let resultError {
        errorOut?.pointee = ffiString(resultError.localizedDescription)
        return nil
    }
    outOpenInPlace?.pointee = actualOpenInPlace
    return ffiString(resultURL?.path)
}
