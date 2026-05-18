# COVERAGE

- Framework: `UniformTypeIdentifiers.framework` (macOS 26.2 SDK)
- Audit basis: public headers plus `UniformTypeIdentifiers.apinotes` for `UTTypeReference` / `UTType.ReferenceType` naming
- Legend: `✅` implemented, `⏭️` skipped/not public SDK surface

## UTType / UTTypeReference

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `UTTypeReference` / `UTType.ReferenceType` | ✅ | `pub type UTType = UTI;` / `pub type UTTypeReference = UTI;` | Mirrors `UniformTypeIdentifiers.apinotes` bridging names. |
| `+typeWithIdentifier:` | ✅ | `UTI::from_identifier` | Swift `UTType.init(_:)`. |
| `+typeWithFilenameExtension:` | ✅ | `UTI::from_filename_extension` | Default-data lookup. |
| `+typeWithFilenameExtension:conformingToType:` | ✅ | `UTI::from_filename_extension_conforming_to` |  |
| `+typeWithMIMEType:` | ✅ | `UTI::from_mime_type` | Default-data lookup. |
| `+typeWithMIMEType:conformingToType:` | ✅ | `UTI::from_mime_type_conforming_to` |  |
| `identifier` | ✅ | `UTI::identifier` |  |
| `preferredFilenameExtension` | ✅ | `UTI::preferred_filename_extension` + `UTI::filename_extensions` |  |
| `preferredMIMEType` | ✅ | `UTI::preferred_mime_type` + `UTI::mime_types` |  |
| `localizedDescription` | ✅ | `UTI::localized_description` |  |
| `version` / Swift `Int?` | ✅ | `UTI::version_number` + compatibility shim `UTI::version` | v0.4 keeps the legacy floating-point accessor and adds an integer-safe API. |
| `referenceURL` | ✅ | `UTI::reference_url` |  |
| `dynamic` / Swift `isDynamic` | ✅ | `UTI::is_dynamic` | Dynamic lookup smoke-tested with unknown extensions. |
| `declared` / Swift `isDeclared` | ✅ | `UTI::is_declared` |  |
| `publicType` / Swift `isPublic` | ✅ | `UTI::is_public_type` + `UTI::is_public` | Bridge now consults the real `isPublic` property instead of inferring from the identifier prefix. |

## UTType (Conformance)

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `-conformsToType:` | ✅ | `UTI::conforms_to` |  |
| `-isSupertypeOfType:` | ✅ | `UTI::is_supertype_of` |  |
| `-isSubtypeOfType:` | ✅ | `UTI::is_subtype_of` |  |
| `supertypes` | ✅ | `UTI::supertypes` | Returns the framework-reported set sorted by identifier for deterministic iteration. |
| Subtype enumeration | ⏭️ | n/a | The public SDK exposes subtype predicates, but no public API enumerates subtypes/children. |

## UTType (UTTagSpecification)

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `+typeWithTag:tagClass:conformingToType:` | ✅ | `UTI::from_tag` |  |
| `+typesWithTag:tagClass:conformingToType:` | ✅ | `UTI::types_with_tag` |  |
| `tags` | ✅ | `UTI::tags` + `UTI::tag_values` |  |
| Filename-extension mapping helpers | ✅ | `UTI::types_for_filename_extension` | Crate convenience over `types_with_tag`. |
| MIME mapping helpers | ✅ | `UTI::types_for_mime_type` | Crate convenience over `types_with_tag`. |
| `OSType` / `FourCharCode` helpers | ✅ | `tag_class::OS_TYPE`, `os_type::*`, `UTI::from_os_type`, `UTI::types_for_os_type`, `UTI::preferred_os_type`, `UTI::os_types` | `com.apple.ostype` is not exported by `UTTagClass.h`, but it is present in live tag dictionaries and is surfaced as crate convenience. |

## UTType (LocalConstants)

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `+exportedTypeWithIdentifier:` | ✅ | `UTI::exported_type_with_identifier` |  |
| `+exportedTypeWithIdentifier:conformingToType:` | ✅ | `UTI::exported_type_with_identifier_conforming_to` |  |
| `+importedTypeWithIdentifier:` | ✅ | `UTI::imported_type_with_identifier` |  |
| `+importedTypeWithIdentifier:conformingToType:` | ✅ | `UTI::imported_type_with_identifier_conforming_to` | Smoke-tested for declared/non-dynamic local types. |

## UTTagClass.h

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `UTTagClassFilenameExtension` | ✅ | `tag_class::FILENAME_EXTENSION` |  |
| `UTTagClassMIMEType` | ✅ | `tag_class::MIME_TYPE` |  |

## UTAdditions.h

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `NSString -stringByAppendingPathComponent:conformingToType:` | ✅ | `additions::append_path_component_conforming_to` |  |
| `NSString -stringByAppendingPathExtensionForType:` | ✅ | `additions::append_path_extension_for_type` |  |
| `NSURL -URLByAppendingPathComponent:conformingToType:` | ✅ | `additions::append_url_path_component_conforming_to` |  |
| `NSURL -URLByAppendingPathExtensionForType:` | ✅ | `additions::append_url_path_extension_for_type` |  |

## NSItemProvider+UTType.h

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `-initWithContentsOfURL:contentType:openInPlace:coordinated:visibility:` | ✅ | `ItemProvider::from_file_path` | Bridge now uses the typed `UTType` overloads directly. |
| `-registerDataRepresentationForContentType:visibility:loadHandler:` | ✅ | `ItemProvider::register_data_representation` |  |
| `-registerFileRepresentationForContentType:visibility:openInPlace:loadHandler:` | ✅ | `ItemProvider::register_file_representation` |  |
| `registeredContentTypes` | ✅ | `ItemProvider::registered_content_types` |  |
| `registeredContentTypesForOpenInPlace` | ✅ | `ItemProvider::registered_content_types_for_open_in_place` |  |
| `-registeredContentTypesConformingToContentType:` | ✅ | `ItemProvider::registered_content_types_conforming_to` |  |
| `-loadDataRepresentationForContentType:completionHandler:` | ✅ | `ItemProvider::load_data_representation` | Bridged to sync with `DispatchSemaphore`. |
| `-loadFileRepresentationForContentType:openInPlace:completionHandler:` | ✅ | `ItemProvider::load_file_representation` | Bridged to sync with `DispatchSemaphore`. |

## UTCoreTypes.h

| Rust constant | Identifier | Status | Notes |
| --- | --- | --- | --- |
| `ITEM` | `public.item` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `CONTENT` | `public.content` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `COMPOSITE_CONTENT` | `public.composite-content` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `DISK_IMAGE` | `public.disk-image` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `DATA` | `public.data` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `DIRECTORY` | `public.directory` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `RESOLVABLE` | `com.apple.resolvable` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `SYMBOLIC_LINK` | `public.symlink` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `EXECUTABLE` | `public.executable` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `MOUNT_POINT` | `com.apple.mount-point` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `ALIAS_FILE` | `com.apple.alias-file` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `URL_BOOKMARK_DATA` | `com.apple.bookmark` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `URL` | `public.url` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `FILE_URL` | `public.file-url` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `TEXT` | `public.text` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `PLAIN_TEXT` | `public.plain-text` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `UTF8_PLAIN_TEXT` | `public.utf8-plain-text` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `UTF16_EXTERNAL_PLAIN_TEXT` | `public.utf16-external-plain-text` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `UTF16_PLAIN_TEXT` | `public.utf16-plain-text` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `DELIMITED_TEXT` | `public.delimited-values-text` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `COMMA_SEPARATED_TEXT` | `public.comma-separated-values-text` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `TAB_SEPARATED_TEXT` | `public.tab-separated-values-text` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `UTF8_TAB_SEPARATED_TEXT` | `public.utf8-tab-separated-values-text` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `RTF` | `public.rtf` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `HTML` | `public.html` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `XML` | `public.xml` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `YAML` | `public.yaml` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `CSS` | `public.css` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `SOURCE_CODE` | `public.source-code` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `ASSEMBLY_LANGUAGE_SOURCE` | `public.assembly-source` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `C_SOURCE` | `public.c-source` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `OBJECTIVE_C_SOURCE` | `public.objective-c-source` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `SWIFT_SOURCE` | `public.swift-source` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `C_PLUS_PLUS_SOURCE` | `public.c-plus-plus-source` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `OBJECTIVE_C_PLUS_PLUS_SOURCE` | `public.objective-c-plus-plus-source` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `C_HEADER` | `public.c-header` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `C_PLUS_PLUS_HEADER` | `public.c-plus-plus-header` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `SCRIPT` | `public.script` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `APPLE_SCRIPT_TEXT` | `com.apple.applescript.text` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `OSA_SCRIPT` | `com.apple.applescript.script` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `OSA_SCRIPT_BUNDLE` | `com.apple.applescript.script-bundle` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `JAVA_SCRIPT` | `com.netscape.javascript-source` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `SHELL_SCRIPT` | `public.shell-script` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `PERL_SCRIPT` | `public.perl-script` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `PYTHON_SCRIPT` | `public.python-script` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `RUBY_SCRIPT` | `public.ruby-script` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `PHP_SCRIPT` | `public.php-script` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `MAKEFILE` | `public.make-source` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `JSON` | `public.json` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `PROPERTY_LIST` | `com.apple.property-list` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `XML_PROPERTY_LIST` | `com.apple.xml-property-list` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `BINARY_PROPERTY_LIST` | `com.apple.binary-property-list` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `PDF` | `com.adobe.pdf` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `RTFD` | `com.apple.rtfd` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `FLAT_RTFD` | `com.apple.flat-rtfd` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `WEB_ARCHIVE` | `com.apple.webarchive` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `IMAGE` | `public.image` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `JPEG` | `public.jpeg` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `TIFF` | `public.tiff` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `GIF` | `com.compuserve.gif` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `PNG` | `public.png` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `ICNS` | `com.apple.icns` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `BMP` | `com.\u006d\u0069\u0063\u0072\u006f\u0073\u006f\u0066\u0074.bmp` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `ICO` | `com.\u006d\u0069\u0063\u0072\u006f\u0073\u006f\u0066\u0074.ico` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `RAW_IMAGE` | `public.camera-raw-image` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `SVG` | `public.svg-image` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `LIVE_PHOTO` | `com.apple.live-photo` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `HEIF` | `public.heif` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `HEIC` | `public.heic` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `HEICS` | `public.heics` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `WEBP` | `org.webmproject.webp` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `EXR` | `com.ilm.openexr-image` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `DNG` | `com.adobe.raw-image` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `JPEG_XL` | `public.jpeg-xl` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `THREE_D_CONTENT` | `public.3d-content` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `USD` | `com.pixar.universal-scene-description` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `USDZ` | `com.pixar.universal-scene-description-mobile` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `REALITY_FILE` | `com.apple.reality` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `SCENEKIT_SCENE` | `com.apple.scenekit.scene` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `AR_REFERENCE_OBJECT` | `com.apple.arobject` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `AUDIOVISUAL_CONTENT` | `public.audiovisual-content` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `MOVIE` | `public.movie` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `VIDEO` | `public.video` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `AUDIO` | `public.audio` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `QUICKTIME_MOVIE` | `com.apple.quicktime-movie` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `MPEG` | `public.mpeg` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `MPEG2_VIDEO` | `public.mpeg-2-video` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `MPEG2_TRANSPORT_STREAM` | `public.mpeg-2-transport-stream` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `MP3` | `public.mp3` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `MPEG4_MOVIE` | `public.mpeg-4` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `MPEG4_AUDIO` | `public.mpeg-4-audio` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `APPLE_PROTECTED_MPEG4_AUDIO` | `com.apple.protected-mpeg-4-audio` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `APPLE_PROTECTED_MPEG4_VIDEO` | `com.apple.protected-mpeg-4-video` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `AVI` | `public.avi` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `AIFF` | `public.aiff-audio` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `WAV` | `com.\u006d\u0069\u0063\u0072\u006f\u0073\u006f\u0066\u0074.waveform-audio` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `MIDI` | `public.midi-audio` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `PLAYLIST` | `public.playlist` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `M3U_PLAYLIST` | `public.m3u-playlist` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `FOLDER` | `public.folder` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `VOLUME` | `public.volume` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `PACKAGE` | `com.apple.package` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `BUNDLE` | `com.apple.bundle` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `PLUGIN_BUNDLE` | `com.apple.plugin` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `SPOTLIGHT_IMPORTER` | `com.apple.metadata-importer` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `QUICK_LOOK_GENERATOR` | `com.apple.quicklook-generator` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `XPC_SERVICE` | `com.apple.xpc-service` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `FRAMEWORK` | `com.apple.framework` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `APPLICATION` | `com.apple.application` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `APPLICATION_BUNDLE` | `com.apple.application-bundle` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `APPLICATION_EXTENSION` | `com.apple.application-and-system-extension` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `UNIX_EXECUTABLE` | `public.unix-executable` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `EXE` | `com.\u006d\u0069\u0063\u0072\u006f\u0073\u006f\u0066\u0074.windows-executable` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `SYSTEM_PREFERENCES_PANE` | `com.apple.systempreference.prefpane` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `ARCHIVE` | `public.archive` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `GZIP` | `org.gnu.gnu-zip-archive` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `BZ2` | `public.bzip2-archive` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `ZIP` | `public.zip-archive` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `APPLE_ARCHIVE` | `com.apple.archive` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `TAR_ARCHIVE` | `public.tar-archive` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `SPREADSHEET` | `public.spreadsheet` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `PRESENTATION` | `public.presentation` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `DATABASE` | `public.database` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `MESSAGE` | `public.message` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `CONTACT` | `public.contact` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `VCARD` | `public.vcard` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `TO_DO_ITEM` | `public.to-do-item` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `CALENDAR_EVENT_ITEM` | `public.calendar-event` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `EMAIL_MESSAGE` | `public.email-message` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `INTERNET_LOCATION` | `com.apple.internet-location` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `INTERNET_SHORTCUT` | `com.apple.internet-location` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `FONT` | `public.font` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `BOOKMARK` | `public.bookmark` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `PKCS12` | `com.rsa.pkcs-12` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `X509_CERTIFICATE` | `public.x509-certificate` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `EPUB` | `org.idpf.epub-container` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `LOG` | `public.log` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `AHAP` | `com.apple.haptics.ahap` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `GEOJSON` | `public.geojson` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `LINK_PRESENTATION_METADATA` | `com.apple.linkpresentation.metadata` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `CALENDAR_EVENT` | `com.apple.ical.vcs` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |
| `KERNEL_EXTENSION` | `com.apple.kernel-extension` | ✅ | Mirrored in `src/core_types.rs` and covered by header/bridge coverage tests. |

## Deferred / skipped

| Item | Status | Reason |
| --- | --- | --- |
| Public subtype enumeration API | ⏭️ | Apple publishes subtype predicates (`isSubtypeOfType:` / `isSupertypeOfType:`) but no public API to enumerate children/subtypes. |

