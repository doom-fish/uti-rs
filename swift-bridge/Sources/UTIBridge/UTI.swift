// UTI Bridge
//
// @_cdecl wrappers around Apple's `UTType` (UniformTypeIdentifiers
// framework). UTType is a Swift `struct` (not a class), so we box it in
// an NSObject wrapper before crossing the FFI as
// `UnsafeMutableRawPointer`. Rust calls `uti_release` to drop ownership.

import Foundation
import UniformTypeIdentifiers

/// Boxes a UTType value so it can cross FFI as a refcounted opaque pointer.
final class UTITypeBox: NSObject {
    let inner: UTType
    init(_ t: UTType) { self.inner = t }
}

private func ffiString(_ s: String?) -> UnsafeMutablePointer<CChar>? {
    guard let s = s else { return nil }
    return strdup(s)
}

@_cdecl("uti_string_free")
public func uti_string_free(_ s: UnsafeMutablePointer<CChar>?) {
    guard let s = s else { return }
    free(s)
}

@_cdecl("uti_release")
public func uti_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr = ptr else { return }
    Unmanaged<UTITypeBox>.fromOpaque(ptr).release()
}

@_cdecl("uti_retain")
public func uti_retain(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let ptr = ptr else { return nil }
    let box = Unmanaged<UTITypeBox>.fromOpaque(ptr).takeUnretainedValue()
    return Unmanaged.passRetained(box).toOpaque()
}

private func makeOpaque(_ t: UTType?) -> UnsafeMutableRawPointer? {
    guard let t = t else { return nil }
    return Unmanaged.passRetained(UTITypeBox(t)).toOpaque()
}

private func unbox(_ ptr: UnsafeMutableRawPointer) -> UTType {
    return Unmanaged<UTITypeBox>.fromOpaque(ptr).takeUnretainedValue().inner
}

// MARK: - Construction

@_cdecl("uti_from_identifier")
public func uti_from_identifier(_ s: UnsafePointer<CChar>) -> UnsafeMutableRawPointer? {
    return makeOpaque(UTType(String(cString: s)))
}

@_cdecl("uti_from_filename_extension")
public func uti_from_filename_extension(_ s: UnsafePointer<CChar>) -> UnsafeMutableRawPointer? {
    return makeOpaque(UTType(filenameExtension: String(cString: s)))
}

@_cdecl("uti_from_mime_type")
public func uti_from_mime_type(_ s: UnsafePointer<CChar>) -> UnsafeMutableRawPointer? {
    return makeOpaque(UTType(mimeType: String(cString: s)))
}

@_cdecl("uti_from_filename_extension_conforming_to")
public func uti_from_filename_extension_conforming_to(
    _ ext: UnsafePointer<CChar>,
    _ supertype: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let supertype = supertype else { return nil }
    return makeOpaque(UTType(filenameExtension: String(cString: ext), conformingTo: unbox(supertype)))
}

// MARK: - Accessors

@_cdecl("uti_identifier")
public func uti_identifier(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    return ffiString(unbox(ptr).identifier)
}

@_cdecl("uti_preferred_filename_extension")
public func uti_preferred_filename_extension(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    return ffiString(unbox(ptr).preferredFilenameExtension)
}

@_cdecl("uti_preferred_mime_type")
public func uti_preferred_mime_type(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    return ffiString(unbox(ptr).preferredMIMEType)
}

@_cdecl("uti_localized_description")
public func uti_localized_description(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    return ffiString(unbox(ptr).localizedDescription)
}

@_cdecl("uti_is_dynamic")
public func uti_is_dynamic(_ ptr: UnsafeMutableRawPointer) -> Bool {
    return unbox(ptr).isDynamic
}

@_cdecl("uti_is_declared")
public func uti_is_declared(_ ptr: UnsafeMutableRawPointer) -> Bool {
    return unbox(ptr).isDeclared
}

@_cdecl("uti_is_public_type")
public func uti_is_public_type(_ ptr: UnsafeMutableRawPointer) -> Bool {
    return unbox(ptr).identifier.hasPrefix("public.")
}

// MARK: - Conformance

@_cdecl("uti_conforms_to")
public func uti_conforms_to(
    _ ptr: UnsafeMutableRawPointer,
    _ other: UnsafeMutableRawPointer
) -> Bool {
    return unbox(ptr).conforms(to: unbox(other))
}

@_cdecl("uti_equals")
public func uti_equals(
    _ ptr: UnsafeMutableRawPointer,
    _ other: UnsafeMutableRawPointer
) -> Bool {
    return unbox(ptr) == unbox(other)
}

// MARK: - Built-in static types
//
// Apple ships ~150 well-known UTType class properties (UTType.png, .jpeg,
// .heic, .pdf, .text, .audio, .image, ...). Expose a single getter that
// switches on the name string so Rust can request any of them.

@_cdecl("uti_well_known")
public func uti_well_known(_ name: UnsafePointer<CChar>) -> UnsafeMutableRawPointer? {
    let n = String(cString: name)
    let t: UTType?
    switch n {
    // Abstract
    case "item": t = .item
    case "content": t = .content
    case "compositeContent": t = .compositeContent
    case "diskImage": t = .diskImage
    case "data": t = .data
    case "directory": t = .directory
    case "package": t = .package
    case "bundle": t = .bundle
    case "executable": t = .executable
    case "archive": t = .archive
    case "log": t = .log
    // Text
    case "text": t = .text
    case "plainText": t = .plainText
    case "utf8PlainText": t = .utf8PlainText
    case "utf16PlainText": t = .utf16PlainText
    case "rtf": t = .rtf
    case "html": t = .html
    case "xml": t = .xml
    case "yaml": t = .yaml
    case "json": t = .json
    case "sourceCode": t = .sourceCode
    case "swiftSource": t = .swiftSource
    case "shellScript": t = .shellScript
    case "pythonScript": t = .pythonScript
    case "rubyScript": t = .rubyScript
    case "phpScript": t = .phpScript
    case "perlScript": t = .perlScript
    case "javaScript": t = .javaScript
    // Images
    case "image": t = .image
    case "png": t = .png
    case "jpeg": t = .jpeg
    case "tiff": t = .tiff
    case "gif": t = .gif
    case "bmp": t = .bmp
    case "ico": t = .ico
    case "svg": t = .svg
    case "webP": t = .webP
    case "rawImage": t = .rawImage
    case "heic": t = .heic
    case "heif": t = .heif
    case "icns": t = .icns
    case "livePhoto": t = .livePhoto
    // Audio + video + movies
    case "audiovisualContent": t = .audiovisualContent
    case "movie": t = .movie
    case "video": t = .video
    case "audio": t = .audio
    case "quickTimeMovie": t = .quickTimeMovie
    case "mpeg": t = .mpeg
    case "mpeg2Video": t = .mpeg2Video
    case "mpeg2TransportStream": t = .mpeg2TransportStream
    case "mp3": t = .mp3
    case "mpeg4Movie": t = .mpeg4Movie
    case "mpeg4Audio": t = .mpeg4Audio
    case "appleProtectedMPEG4Audio": t = .appleProtectedMPEG4Audio
    case "appleProtectedMPEG4Video": t = .appleProtectedMPEG4Video
    case "avi": t = .avi
    case "aiff": t = .aiff
    case "wav": t = .wav
    case "midi": t = .midi
    // Documents + container formats
    case "pdf": t = .pdf
    case "epub": t = .epub
    case "presentation": t = .presentation
    case "spreadsheet": t = .spreadsheet
    case "database": t = .database
    case "vCard": t = .vCard
    case "calendarEvent": t = .calendarEvent
    case "emailMessage": t = .emailMessage
    case "internetLocation": t = .internetLocation
    // Web + URLs
    case "url": t = .url
    case "fileURL": t = .fileURL
    // Programs
    case "application": t = .application
    case "applicationBundle": t = .applicationBundle
    case "framework": t = .framework
    case "unixExecutable": t = .unixExecutable
    case "x509Certificate": t = .x509Certificate
    // Archives
    case "zip": t = .zip
    case "gzip": t = .gzip
    case "bz2": t = .bz2
    // Filesystem + symlinks
    case "symbolicLink": t = .symbolicLink
    case "alias": t = .aliasFile
    case "aliasFile": t = .aliasFile
    case "volume": t = .volume
    case "mountPoint": t = .mountPoint
    case "folder": t = .folder
    // Pasteboard
    case "pkcs12": t = .pkcs12
    default: t = nil
    }
    return makeOpaque(t)
}
