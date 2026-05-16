import Foundation
import UniformTypeIdentifiers

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
