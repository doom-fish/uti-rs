//
//  UTI Bridge
//
// @_cdecl wrappers around Apple's `UTType` (`UniformTypeIdentifiers`
// framework). `UTType` is a Swift `struct`, so we box it in an NSObject
// wrapper before crossing the FFI as `UnsafeMutableRawPointer`.

import Dispatch
import Foundation
import UniformTypeIdentifiers

final class UTITypeBox: NSObject {
    let inner: UTType

    init(_ type: UTType) {
        self.inner = type
    }
}

final class ItemProviderBox: NSObject {
    let inner: NSItemProvider

    init(_ provider: NSItemProvider) {
        self.inner = provider
    }
}

private let wellKnownIdentifiers: [String: String] = [
    "item": "public.item",
    "content": "public.content",
    "compositeContent": "public.composite-content",
    "diskImage": "public.disk-image",
    "data": "public.data",
    "directory": "public.directory",
    "resolvable": "com.apple.resolvable",
    "symbolicLink": "public.symlink",
    "executable": "public.executable",
    "mountPoint": "com.apple.mount-point",
    "aliasFile": "com.apple.alias-file",
    "urlBookmarkData": "com.apple.bookmark",
    "url": "public.url",
    "fileURL": "public.file-url",
    "text": "public.text",
    "plainText": "public.plain-text",
    "utf8PlainText": "public.utf8-plain-text",
    "utf16ExternalPlainText": "public.utf16-external-plain-text",
    "utf16PlainText": "public.utf16-plain-text",
    "delimitedText": "public.delimited-values-text",
    "commaSeparatedText": "public.comma-separated-values-text",
    "tabSeparatedText": "public.tab-separated-values-text",
    "utf8TabSeparatedText": "public.utf8-tab-separated-values-text",
    "rtf": "public.rtf",
    "html": "public.html",
    "xml": "public.xml",
    "yaml": "public.yaml",
    "css": "public.css",
    "sourceCode": "public.source-code",
    "assemblyLanguageSource": "public.assembly-source",
    "cSource": "public.c-source",
    "objectiveCSource": "public.objective-c-source",
    "swiftSource": "public.swift-source",
    "cPlusPlusSource": "public.c-plus-plus-source",
    "objectiveCPlusPlusSource": "public.objective-c-plus-plus-source",
    "cHeader": "public.c-header",
    "cPlusPlusHeader": "public.c-plus-plus-header",
    "script": "public.script",
    "appleScript": "com.apple.applescript.text",
    "osaScript": "com.apple.applescript.script",
    "osaScriptBundle": "com.apple.applescript.script-bundle",
    "javaScript": "com.netscape.javascript-source",
    "shellScript": "public.shell-script",
    "perlScript": "public.perl-script",
    "pythonScript": "public.python-script",
    "rubyScript": "public.ruby-script",
    "phpScript": "public.php-script",
    "makefile": "public.make-source",
    "json": "public.json",
    "propertyList": "com.apple.property-list",
    "xmlPropertyList": "com.apple.xml-property-list",
    "binaryPropertyList": "com.apple.binary-property-list",
    "pdf": "com.adobe.pdf",
    "rtfd": "com.apple.rtfd",
    "flatRTFD": "com.apple.flat-rtfd",
    "webArchive": "com.apple.webarchive",
    "image": "public.image",
    "jpeg": "public.jpeg",
    "tiff": "public.tiff",
    "gif": "com.compuserve.gif",
    "png": "public.png",
    "icns": "com.apple.icns",
    "bmp": "com.microsoft.bmp",
    "ico": "com.microsoft.ico",
    "rawImage": "public.camera-raw-image",
    "svg": "public.svg-image",
    "livePhoto": "com.apple.live-photo",
    "heif": "public.heif",
    "heic": "public.heic",
    "heics": "public.heics",
    "webP": "org.webmproject.webp",
    "exr": "com.ilm.openexr-image",
    "dng": "com.adobe.raw-image",
    "jpegXL": "public.jpeg-xl",
    "threeDContent": "public.3d-content",
    "usd": "com.pixar.universal-scene-description",
    "usdz": "com.pixar.universal-scene-description-mobile",
    "realityFile": "com.apple.reality",
    "sceneKitScene": "com.apple.scenekit.scene",
    "arReferenceObject": "com.apple.arobject",
    "audiovisualContent": "public.audiovisual-content",
    "movie": "public.movie",
    "video": "public.video",
    "audio": "public.audio",
    "quickTimeMovie": "com.apple.quicktime-movie",
    "mpeg": "public.mpeg",
    "mpeg2Video": "public.mpeg-2-video",
    "mpeg2TransportStream": "public.mpeg-2-transport-stream",
    "mp3": "public.mp3",
    "mpeg4Movie": "public.mpeg-4",
    "mpeg4Audio": "public.mpeg-4-audio",
    "appleProtectedMPEG4Audio": "com.apple.protected-mpeg-4-audio",
    "appleProtectedMPEG4Video": "com.apple.protected-mpeg-4-video",
    "avi": "public.avi",
    "aiff": "public.aiff-audio",
    "wav": "com.microsoft.waveform-audio",
    "midi": "public.midi-audio",
    "playlist": "public.playlist",
    "m3uPlaylist": "public.m3u-playlist",
    "folder": "public.folder",
    "volume": "public.volume",
    "package": "com.apple.package",
    "bundle": "com.apple.bundle",
    "pluginBundle": "com.apple.plugin",
    "spotlightImporter": "com.apple.metadata-importer",
    "quickLookGenerator": "com.apple.quicklook-generator",
    "xpcService": "com.apple.xpc-service",
    "framework": "com.apple.framework",
    "application": "com.apple.application",
    "applicationBundle": "com.apple.application-bundle",
    "applicationExtension": "com.apple.application-and-system-extension",
    "unixExecutable": "public.unix-executable",
    "exe": "com.microsoft.windows-executable",
    "systemPreferencesPane": "com.apple.systempreference.prefpane",
    "archive": "public.archive",
    "gzip": "org.gnu.gnu-zip-archive",
    "bz2": "public.bzip2-archive",
    "zip": "public.zip-archive",
    "appleArchive": "com.apple.archive",
    "tarArchive": "public.tar-archive",
    "spreadsheet": "public.spreadsheet",
    "presentation": "public.presentation",
    "database": "public.database",
    "message": "public.message",
    "contact": "public.contact",
    "vCard": "public.vcard",
    "toDoItem": "public.to-do-item",
    "calendarEvent": "public.calendar-event",
    "emailMessage": "public.email-message",
    "internetLocation": "com.apple.internet-location",
    "internetShortcut": "com.apple.internet-location",
    "font": "public.font",
    "bookmark": "public.bookmark",
    "pkcs12": "com.rsa.pkcs-12",
    "x509Certificate": "public.x509-certificate",
    "epub": "org.idpf.epub-container",
    "log": "public.log",
    "ahap": "com.apple.haptics.ahap",
    "geoJSON": "public.geojson",
    "linkPresentationMetadata": "com.apple.linkpresentation.metadata",
    "alias": "com.apple.alias-file",
    "arkitRealityFile": "com.apple.reality",
    "appleScriptText": "com.apple.applescript.text",
    "calendarEventItem": "public.calendar-event",
]

private func ffiString(_ string: String?) -> UnsafeMutablePointer<CChar>? {
    guard let string else { return nil }
    return strdup(string)
}

private func ffiJoinedStrings(_ strings: [String]) -> UnsafeMutablePointer<CChar>? {
    ffiString(strings.joined(separator: "\n"))
}

private func ffiTagMap(_ tags: [String: [String]]) -> UnsafeMutablePointer<CChar>? {
    let lines = tags.keys.sorted().map { key in
        ([key] + (tags[key] ?? []).sorted()).joined(separator: "\t")
    }
    return ffiJoinedStrings(lines)
}

private func ffiData(_ data: Data?) -> UnsafeMutablePointer<UInt8>? {
    guard let data else { return nil }
    let pointer = UnsafeMutablePointer<UInt8>.allocate(capacity: max(data.count, 1))
    if !data.isEmpty {
        data.copyBytes(to: pointer, count: data.count)
    }
    return pointer
}

@_cdecl("uti_string_free")
public func uti_string_free(_ string: UnsafeMutablePointer<CChar>?) {
    guard let string else { return }
    free(string)
}

@_cdecl("uti_bytes_free")
public func uti_bytes_free(_ bytes: UnsafeMutablePointer<UInt8>?, _ len: Int) {
    _ = len
    guard let bytes else { return }
    bytes.deallocate()
}

@_cdecl("uti_release")
public func uti_release(_ pointer: UnsafeMutableRawPointer?) {
    guard let pointer else { return }
    Unmanaged<UTITypeBox>.fromOpaque(pointer).release()
}

@_cdecl("uti_retain")
public func uti_retain(_ pointer: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let pointer else { return nil }
    let box = Unmanaged<UTITypeBox>.fromOpaque(pointer).takeUnretainedValue()
    return Unmanaged.passRetained(box).toOpaque()
}

@_cdecl("item_provider_release")
public func item_provider_release(_ pointer: UnsafeMutableRawPointer?) {
    guard let pointer else { return }
    Unmanaged<ItemProviderBox>.fromOpaque(pointer).release()
}

@_cdecl("item_provider_retain")
public func item_provider_retain(_ pointer: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let pointer else { return nil }
    let box = Unmanaged<ItemProviderBox>.fromOpaque(pointer).takeUnretainedValue()
    return Unmanaged.passRetained(box).toOpaque()
}

private func makeOpaque(_ type: UTType?) -> UnsafeMutableRawPointer? {
    guard let type else { return nil }
    return Unmanaged.passRetained(UTITypeBox(type)).toOpaque()
}

private func unbox(_ pointer: UnsafeMutableRawPointer) -> UTType {
    Unmanaged<UTITypeBox>.fromOpaque(pointer).takeUnretainedValue().inner
}

private func makeProviderOpaque(_ provider: NSItemProvider) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(ItemProviderBox(provider)).toOpaque()
}

private func unboxProvider(_ pointer: UnsafeMutableRawPointer) -> NSItemProvider {
    Unmanaged<ItemProviderBox>.fromOpaque(pointer).takeUnretainedValue().inner
}

private func visibility(from rawValue: Int64) -> NSItemProviderRepresentationVisibility {
    NSItemProviderRepresentationVisibility(rawValue: Int(rawValue)) ?? .all
}

@_cdecl("uti_from_identifier")
public func uti_from_identifier(_ identifier: UnsafePointer<CChar>) -> UnsafeMutableRawPointer? {
    makeOpaque(UTType(String(cString: identifier)))
}

@_cdecl("uti_from_filename_extension")
public func uti_from_filename_extension(_ ext: UnsafePointer<CChar>) -> UnsafeMutableRawPointer? {
    makeOpaque(UTType(filenameExtension: String(cString: ext)))
}

@_cdecl("uti_from_filename_extension_conforming_to")
public func uti_from_filename_extension_conforming_to(
    _ ext: UnsafePointer<CChar>,
    _ supertype: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let supertype else { return nil }
    return makeOpaque(UTType(filenameExtension: String(cString: ext), conformingTo: unbox(supertype)))
}

@_cdecl("uti_from_mime_type")
public func uti_from_mime_type(_ mime: UnsafePointer<CChar>) -> UnsafeMutableRawPointer? {
    makeOpaque(UTType(mimeType: String(cString: mime)))
}

@_cdecl("uti_from_mime_type_conforming_to")
public func uti_from_mime_type_conforming_to(
    _ mime: UnsafePointer<CChar>,
    _ supertype: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let supertype else { return nil }
    return makeOpaque(UTType(mimeType: String(cString: mime), conformingTo: unbox(supertype)))
}

@_cdecl("uti_from_tag")
public func uti_from_tag(
    _ tag: UnsafePointer<CChar>,
    _ tagClass: UnsafePointer<CChar>,
    _ supertype: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    let tagClass = UTTagClass(rawValue: String(cString: tagClass))
    let supertype = supertype.map(unbox)
    return makeOpaque(UTType(tag: String(cString: tag), tagClass: tagClass, conformingTo: supertype))
}

@_cdecl("uti_types_with_tag")
public func uti_types_with_tag(
    _ tag: UnsafePointer<CChar>,
    _ tagClass: UnsafePointer<CChar>,
    _ supertype: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    let tagClass = UTTagClass(rawValue: String(cString: tagClass))
    let types = UTType.types(
        tag: String(cString: tag),
        tagClass: tagClass,
        conformingTo: supertype.map(unbox)
    )
    .map(\.identifier)
    .sorted()
    return ffiJoinedStrings(types)
}

@_cdecl("uti_exported_type_with_identifier")
public func uti_exported_type_with_identifier(_ identifier: UnsafePointer<CChar>) -> UnsafeMutableRawPointer? {
    makeOpaque(UTType(exportedAs: String(cString: identifier)))
}

@_cdecl("uti_exported_type_with_identifier_conforming_to")
public func uti_exported_type_with_identifier_conforming_to(
    _ identifier: UnsafePointer<CChar>,
    _ parentType: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let parentType else { return nil }
    return makeOpaque(UTType(exportedAs: String(cString: identifier), conformingTo: unbox(parentType)))
}

@_cdecl("uti_imported_type_with_identifier")
public func uti_imported_type_with_identifier(_ identifier: UnsafePointer<CChar>) -> UnsafeMutableRawPointer? {
    makeOpaque(UTType(importedAs: String(cString: identifier)))
}

@_cdecl("uti_imported_type_with_identifier_conforming_to")
public func uti_imported_type_with_identifier_conforming_to(
    _ identifier: UnsafePointer<CChar>,
    _ parentType: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let parentType else { return nil }
    return makeOpaque(UTType(importedAs: String(cString: identifier), conformingTo: unbox(parentType)))
}

@_cdecl("uti_identifier")
public func uti_identifier(_ pointer: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    ffiString(unbox(pointer).identifier)
}

@_cdecl("uti_preferred_filename_extension")
public func uti_preferred_filename_extension(_ pointer: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    ffiString(unbox(pointer).preferredFilenameExtension)
}

@_cdecl("uti_preferred_mime_type")
public func uti_preferred_mime_type(_ pointer: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    ffiString(unbox(pointer).preferredMIMEType)
}

@_cdecl("uti_localized_description")
public func uti_localized_description(_ pointer: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    ffiString(unbox(pointer).localizedDescription)
}

@_cdecl("uti_version")
public func uti_version(
    _ pointer: UnsafeMutableRawPointer,
    _ outValue: UnsafeMutablePointer<Double>?
) -> Bool {
    guard let version = unbox(pointer).version, let outValue else {
        return false
    }
    outValue.pointee = Double(version)
    return true
}

@_cdecl("uti_reference_url")
public func uti_reference_url(_ pointer: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    ffiString(unbox(pointer).referenceURL?.absoluteString)
}

@_cdecl("uti_tags")
public func uti_tags(_ pointer: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let tags = Dictionary(uniqueKeysWithValues: unbox(pointer).tags.map { ($0.key.rawValue, $0.value) })
    return ffiTagMap(tags)
}

@_cdecl("uti_is_dynamic")
public func uti_is_dynamic(_ pointer: UnsafeMutableRawPointer) -> Bool {
    unbox(pointer).isDynamic
}

@_cdecl("uti_is_declared")
public func uti_is_declared(_ pointer: UnsafeMutableRawPointer) -> Bool {
    unbox(pointer).isDeclared
}

@_cdecl("uti_is_public_type")
public func uti_is_public_type(_ pointer: UnsafeMutableRawPointer) -> Bool {
    unbox(pointer).identifier.hasPrefix("public.")
}

@_cdecl("uti_conforms_to")
public func uti_conforms_to(
    _ pointer: UnsafeMutableRawPointer,
    _ other: UnsafeMutableRawPointer
) -> Bool {
    unbox(pointer).conforms(to: unbox(other))
}

@_cdecl("uti_is_supertype_of")
public func uti_is_supertype_of(
    _ pointer: UnsafeMutableRawPointer,
    _ other: UnsafeMutableRawPointer
) -> Bool {
    unbox(pointer).isSupertype(of: unbox(other))
}

@_cdecl("uti_is_subtype_of")
public func uti_is_subtype_of(
    _ pointer: UnsafeMutableRawPointer,
    _ other: UnsafeMutableRawPointer
) -> Bool {
    unbox(pointer).isSubtype(of: unbox(other))
}

@_cdecl("uti_supertypes")
public func uti_supertypes(_ pointer: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    ffiJoinedStrings(unbox(pointer).supertypes.map(\.identifier).sorted())
}

@_cdecl("uti_equals")
public func uti_equals(
    _ pointer: UnsafeMutableRawPointer,
    _ other: UnsafeMutableRawPointer
) -> Bool {
    unbox(pointer) == unbox(other)
}

@_cdecl("uti_string_appending_path_component_conforming_to")
public func uti_string_appending_path_component_conforming_to(
    _ base: UnsafePointer<CChar>,
    _ partial: UnsafePointer<CChar>,
    _ contentType: UnsafeMutableRawPointer
) -> UnsafeMutablePointer<CChar>? {
    let base = String(cString: base) as NSString
    return ffiString(base.appendingPathComponent(String(cString: partial), conformingTo: unbox(contentType)))
}

@_cdecl("uti_string_appending_path_extension_for_type")
public func uti_string_appending_path_extension_for_type(
    _ base: UnsafePointer<CChar>,
    _ contentType: UnsafeMutableRawPointer
) -> UnsafeMutablePointer<CChar>? {
    let base = String(cString: base) as NSString
    return ffiString(base.appendingPathExtension(for: unbox(contentType)))
}

@_cdecl("uti_url_appending_path_component_conforming_to")
public func uti_url_appending_path_component_conforming_to(
    _ baseURL: UnsafePointer<CChar>,
    _ partial: UnsafePointer<CChar>,
    _ contentType: UnsafeMutableRawPointer
) -> UnsafeMutablePointer<CChar>? {
    guard let baseURL = URL(string: String(cString: baseURL)) else { return nil }
    return ffiString(
        baseURL.appendingPathComponent(String(cString: partial), conformingTo: unbox(contentType)).absoluteString
    )
}

@_cdecl("uti_url_appending_path_extension_for_type")
public func uti_url_appending_path_extension_for_type(
    _ baseURL: UnsafePointer<CChar>,
    _ contentType: UnsafeMutableRawPointer
) -> UnsafeMutablePointer<CChar>? {
    guard let baseURL = URL(string: String(cString: baseURL)) else { return nil }
    return ffiString(baseURL.appendingPathExtension(for: unbox(contentType)).absoluteString)
}

@_cdecl("uti_well_known")
public func uti_well_known(_ name: UnsafePointer<CChar>) -> UnsafeMutableRawPointer? {
    let key = String(cString: name)
    guard let identifier = wellKnownIdentifiers[key] else { return nil }
    return makeOpaque(UTType(identifier))
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
        forTypeIdentifier: unbox(contentType).identifier,
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
    let fileOptions: NSItemProviderFileOptions = openInPlace ? .openInPlace : []
    unboxProvider(provider).registerFileRepresentation(
        forTypeIdentifier: unbox(contentType).identifier,
        fileOptions: fileOptions,
        visibility: visibility(from: visibilityRaw)
    ) { completion in
        completion(fileURL, coordinated, nil)
        return nil
    }
}

@_cdecl("item_provider_registered_type_identifiers")
public func item_provider_registered_type_identifiers(
    _ provider: UnsafeMutableRawPointer
) -> UnsafeMutablePointer<CChar>? {
    ffiJoinedStrings(unboxProvider(provider).registeredTypeIdentifiers)
}

@_cdecl("item_provider_registered_type_identifiers_with_file_options")
public func item_provider_registered_type_identifiers_with_file_options(
    _ provider: UnsafeMutableRawPointer,
    _ openInPlace: Bool
) -> UnsafeMutablePointer<CChar>? {
    let fileOptions: NSItemProviderFileOptions = openInPlace ? .openInPlace : []
    return ffiJoinedStrings(unboxProvider(provider).registeredTypeIdentifiers(fileOptions: fileOptions))
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
    _ = unboxProvider(provider).loadDataRepresentation(forTypeIdentifier: unbox(contentType).identifier) { data, error in
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
    let typeIdentifier = unbox(contentType).identifier
    if openInPlace {
        _ = unboxProvider(provider).loadInPlaceFileRepresentation(forTypeIdentifier: typeIdentifier) { url, isInPlace, error in
            resultURL = url
            actualOpenInPlace = isInPlace
            resultError = error
            semaphore.signal()
        }
    } else {
        _ = unboxProvider(provider).loadFileRepresentation(forTypeIdentifier: typeIdentifier) { url, error in
            resultURL = url
            actualOpenInPlace = false
            resultError = error
            semaphore.signal()
        }
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
