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
    let lock = NSLock()
    var finished = false
    var abandoned = false
    var result: Data?
    var error: Error?
}

final class FileRepresentationLoadState: @unchecked Sendable {
    let lock = NSLock()
    var finished = false
    var abandoned = false
    var path: UnsafeMutablePointer<CChar>?
    var openInPlace = false
    var error: Error?
}

final class CallbackContext: @unchecked Sendable {
    let pointer: UnsafeMutableRawPointer

    init(_ pointer: UnsafeMutableRawPointer) {
        self.pointer = pointer
    }
}

let mainThreadLoadMessage =
    "synchronous item-provider loads can't block the main thread; use the async API or call from another thread"

func ffiFileSystemPath(_ url: URL) -> UnsafeMutablePointer<CChar>? {
    url.withUnsafeFileSystemRepresentation { path in
        path.flatMap { strdup($0) }
    }
}

func directoryURL(_ path: UnsafePointer<CChar>) -> URL {
    URL(fileURLWithPath: String(cString: path), isDirectory: true)
}

func keepLoadedFile(_ url: URL, isInPlace: Bool, in destination: URL) throws -> URL {
    if isInPlace {
        return url
    }
    let target = destination.appendingPathComponent(url.lastPathComponent)
    do {
        try FileManager.default.moveItem(at: url, to: target)
    } catch {
        try FileManager.default.copyItem(at: url, to: target)
    }
    return target
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
    let data = bytes.map { Data(bytes: $0, count: len) } ?? Data()
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
    ffiJSON(unboxProvider(provider).registeredContentTypes.map(\.identifier))
}

@_cdecl("item_provider_registered_type_identifiers_with_file_options")
public func item_provider_registered_type_identifiers_with_file_options(
    _ provider: UnsafeMutableRawPointer,
    _ openInPlace: Bool
) -> UnsafeMutablePointer<CChar>? {
    let contentTypes = openInPlace
        ? unboxProvider(provider).registeredContentTypesForOpenInPlace
        : unboxProvider(provider).registeredContentTypes
    return ffiJSON(contentTypes.map(\.identifier))
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
    if Thread.isMainThread {
        errorOut?.pointee = ffiString(mainThreadLoadMessage)
        return nil
    }
    let semaphore = DispatchSemaphore(value: 0)
    let state = DataRepresentationLoadState()
    let progress = unboxProvider(provider).loadDataRepresentation(for: unbox(contentType)) { data, error in
        state.lock.lock()
        defer { state.lock.unlock() }
        guard !state.abandoned else { return }
        state.result = data
        state.error = error
        state.finished = true
        semaphore.signal()
    }
    if semaphore.wait(timeout: .now() + 30) == .timedOut {
        state.lock.lock()
        let finished = state.finished
        if !finished {
            state.abandoned = true
        }
        state.lock.unlock()
        if !finished {
            progress.cancel()
            errorOut?.pointee = ffiString("loadDataRepresentation timed out")
            return nil
        }
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
    _ destinationDirectory: UnsafePointer<CChar>,
    _ outOpenInPlace: UnsafeMutablePointer<Bool>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    errorOut?.pointee = nil
    outOpenInPlace?.pointee = false
    if Thread.isMainThread {
        errorOut?.pointee = ffiString(mainThreadLoadMessage)
        return nil
    }
    let destination = directoryURL(destinationDirectory)
    let semaphore = DispatchSemaphore(value: 0)
    let state = FileRepresentationLoadState()
    let progress = unboxProvider(provider).loadFileRepresentation(
        for: unbox(contentType),
        openInPlace: openInPlace
    ) { url, isInPlace, error in
        state.lock.lock()
        defer { state.lock.unlock() }
        guard !state.abandoned else { return }
        if let error {
            state.error = error
        } else if let url {
            do {
                state.path = ffiFileSystemPath(try keepLoadedFile(url, isInPlace: isInPlace, in: destination))
                state.openInPlace = isInPlace
            } catch {
                state.error = error
            }
        }
        state.finished = true
        semaphore.signal()
    }
    if semaphore.wait(timeout: .now() + 30) == .timedOut {
        state.lock.lock()
        let finished = state.finished
        if !finished {
            state.abandoned = true
        }
        state.lock.unlock()
        if !finished {
            progress.cancel()
            errorOut?.pointee = ffiString("loadFileRepresentation timed out")
            return nil
        }
    }
    if let resultError = state.error {
        errorOut?.pointee = ffiString(resultError.localizedDescription)
        return nil
    }
    outOpenInPlace?.pointee = state.openInPlace
    return state.path
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
    let context = CallbackContext(ctx)

    _ = itemProvider.loadDataRepresentation(for: unbox(contentType)) { data, error in
        defer { unmanaged.release() }
        if let error {
            error.localizedDescription.withCString { cb(nil, 0, $0, context.pointer) }
            return
        }
        guard let data else {
            "loadDataRepresentation returned no data".withCString {
                cb(nil, 0, $0, context.pointer)
            }
            return
        }
        data.withUnsafeBytes { rawBuffer in
            let bytes = rawBuffer.bindMemory(to: UInt8.self)
            cb(bytes.baseAddress, data.count, nil, context.pointer)
        }
    }
}

@_cdecl("item_provider_load_file_representation_async")
public func item_provider_load_file_representation_async(
    _ provider: UnsafeMutableRawPointer?,
    _ contentType: UnsafeMutableRawPointer?,
    _ openInPlace: Bool,
    _ destinationDirectory: UnsafePointer<CChar>?,
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
    guard let destinationDirectory else {
        "missing destination directory".withCString { cb(nil, false, $0, ctx) }
        return
    }

    let unmanaged = unmanagedProviderBox(provider)
    _ = unmanaged.retain()
    let itemProvider = unmanaged.takeUnretainedValue().inner
    let destination = directoryURL(destinationDirectory)
    let context = CallbackContext(ctx)

    _ = itemProvider.loadFileRepresentation(
        for: unbox(contentType),
        openInPlace: openInPlace
    ) { url, isInPlace, error in
        defer { unmanaged.release() }
        if let error {
            error.localizedDescription.withCString { cb(nil, false, $0, context.pointer) }
            return
        }
        guard let url else {
            "loadFileRepresentation returned no file URL".withCString {
                cb(nil, false, $0, context.pointer)
            }
            return
        }
        do {
            let kept = try keepLoadedFile(url, isInPlace: isInPlace, in: destination)
            kept.withUnsafeFileSystemRepresentation { path in
                if let path {
                    cb(path, isInPlace, nil, context.pointer)
                } else {
                    "loaded file URL has no file-system path".withCString {
                        cb(nil, false, $0, context.pointer)
                    }
                }
            }
        } catch {
            error.localizedDescription.withCString { cb(nil, false, $0, context.pointer) }
        }
    }
}
