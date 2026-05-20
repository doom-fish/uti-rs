# uti-rs coverage audit v2 (vs MacOSX26.5.sdk)

SDK_PUBLIC_SYMBOLS: 180
VERIFIED: 180
GAPS: 0
EXEMPT: 0
COVERAGE_PCT: 100.00%

Re-verified the member-level Objective-C surface in `UTType.h`, `UTAdditions.h`, `NSItemProvider+UTType.h`, the exported constants in `UTTagClass.h` + `UTCoreTypes.h`, and the public `UTTypeReference` apinotes alias against MacOSX26.5.sdk. Excluded the NS_UNAVAILABLE `UTType` initializers (`new`/`init`). Each SDK symbol is confirmed present in the crate's wrapper via Rust safe API (`src/**`) and Swift bridge (`swift-bridge/Sources/**`).

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `UTTypeReference / UTType.ReferenceType` | apinotes alias | `UniformTypeIdentifiers.apinotes` | `pub type UTType = UTI;` / `pub type UTTypeReference = UTI;` |
| `+[UTType typeWithIdentifier:]` | class method | `UTType.h` | `UTI::from_identifier` |
| `+[UTType typeWithFilenameExtension:]` | class method | `UTType.h` | `UTI::from_filename_extension` |
| `+[UTType typeWithFilenameExtension:conformingToType:]` | class method | `UTType.h` | `UTI::from_filename_extension_conforming_to` |
| `+[UTType typeWithMIMEType:]` | class method | `UTType.h` | `UTI::from_mime_type` |
| `+[UTType typeWithMIMEType:conformingToType:]` | class method | `UTType.h` | `UTI::from_mime_type_conforming_to` |
| `UTType.identifier` | property | `UTType.h` | `UTI::identifier` |
| `UTType.preferredFilenameExtension` | property | `UTType.h` | `UTI::preferred_filename_extension` + `UTI::filename_extensions` |
| `UTType.preferredMIMEType` | property | `UTType.h` | `UTI::preferred_mime_type` + `UTI::mime_types` |
| `UTType.localizedDescription` | property | `UTType.h` | `UTI::localized_description` |
| `UTType.version` | property | `UTType.h` | `UTI::version_number` + compatibility shim `UTI::version` |
| `UTType.referenceURL` | property | `UTType.h` | `UTI::reference_url` |
| `UTType.dynamic` | property | `UTType.h` | `UTI::is_dynamic` |
| `UTType.declared` | property | `UTType.h` | `UTI::is_declared` |
| `UTType.publicType` | property | `UTType.h` | `UTI::is_public_type` + `UTI::is_public` |
| `-[UTType conformsToType:]` | instance method | `UTType.h` | `UTI::conforms_to` |
| `-[UTType isSupertypeOfType:]` | instance method | `UTType.h` | `UTI::is_supertype_of` |
| `-[UTType isSubtypeOfType:]` | instance method | `UTType.h` | `UTI::is_subtype_of` |
| `UTType.supertypes` | property | `UTType.h` | `UTI::supertypes` |
| `+[UTType typeWithTag:tagClass:conformingToType:]` | class method | `UTType.h` | `UTI::from_tag` |
| `+[UTType typesWithTag:tagClass:conformingToType:]` | class method | `UTType.h` | `UTI::types_with_tag` |
| `UTType.tags` | property | `UTType.h` | `UTI::tags` + `UTI::tag_values` |
| `+[UTType exportedTypeWithIdentifier:]` | class method | `UTType.h` | `UTI::exported_type_with_identifier` |
| `+[UTType exportedTypeWithIdentifier:conformingToType:]` | class method | `UTType.h` | `UTI::exported_type_with_identifier_conforming_to` |
| `+[UTType importedTypeWithIdentifier:]` | class method | `UTType.h` | `UTI::imported_type_with_identifier` |
| `+[UTType importedTypeWithIdentifier:conformingToType:]` | class method | `UTType.h` | `UTI::imported_type_with_identifier_conforming_to` |
| `UTTagClassFilenameExtension` | extern const | `UTTagClass.h` | `tag_class::FILENAME_EXTENSION` |
| `UTTagClassMIMEType` | extern const | `UTTagClass.h` | `tag_class::MIME_TYPE` |
| `-[NSString stringByAppendingPathComponent:conformingToType:]` | instance method | `UTAdditions.h` | `additions::append_path_component_conforming_to` |
| `-[NSString stringByAppendingPathExtensionForType:]` | instance method | `UTAdditions.h` | `additions::append_path_extension_for_type` |
| `-[NSURL URLByAppendingPathComponent:conformingToType:]` | instance method | `UTAdditions.h` | `additions::append_url_path_component_conforming_to` |
| `-[NSURL URLByAppendingPathExtensionForType:]` | instance method | `UTAdditions.h` | `additions::append_url_path_extension_for_type` |
| `-[NSItemProvider initWithContentsOfURL:contentType:openInPlace:coordinated:visibility:]` | instance method | `NSItemProvider+UTType.h` | `ItemProvider::from_file_path` |
| `-[NSItemProvider registerDataRepresentationForContentType:visibility:loadHandler:]` | instance method | `NSItemProvider+UTType.h` | `ItemProvider::register_data_representation` |
| `-[NSItemProvider registerFileRepresentationForContentType:visibility:openInPlace:loadHandler:]` | instance method | `NSItemProvider+UTType.h` | `ItemProvider::register_file_representation` |
| `NSItemProvider.registeredContentTypes` | property | `NSItemProvider+UTType.h` | `ItemProvider::registered_content_types` |
| `NSItemProvider.registeredContentTypesForOpenInPlace` | property | `NSItemProvider+UTType.h` | `ItemProvider::registered_content_types_for_open_in_place` |
| `-[NSItemProvider registeredContentTypesConformingToContentType:]` | instance method | `NSItemProvider+UTType.h` | `ItemProvider::registered_content_types_conforming_to` |
| `-[NSItemProvider loadDataRepresentationForContentType:completionHandler:]` | instance method | `NSItemProvider+UTType.h` | `ItemProvider::load_data_representation` |
| `-[NSItemProvider loadFileRepresentationForContentType:openInPlace:completionHandler:]` | instance method | `NSItemProvider+UTType.h` | `ItemProvider::load_file_representation` |
| `UTTypeItem` (`public.item`) | extern const | `UTCoreTypes.h` | `core_types::ITEM` |
| `UTTypeContent` (`public.content`) | extern const | `UTCoreTypes.h` | `core_types::CONTENT` |
| `UTTypeCompositeContent` (`public.composite-content`) | extern const | `UTCoreTypes.h` | `core_types::COMPOSITE_CONTENT` |
| `UTTypeDiskImage` (`public.disk-image`) | extern const | `UTCoreTypes.h` | `core_types::DISK_IMAGE` |
| `UTTypeData` (`public.data`) | extern const | `UTCoreTypes.h` | `core_types::DATA` |
| `UTTypeDirectory` (`public.directory`) | extern const | `UTCoreTypes.h` | `core_types::DIRECTORY` |
| `UTTypeResolvable` (`com.apple.resolvable`) | extern const | `UTCoreTypes.h` | `core_types::RESOLVABLE` |
| `UTTypeSymbolicLink` (`public.symlink`) | extern const | `UTCoreTypes.h` | `core_types::SYMBOLIC_LINK` |
| `UTTypeExecutable` (`public.executable`) | extern const | `UTCoreTypes.h` | `core_types::EXECUTABLE` |
| `UTTypeMountPoint` (`com.apple.mount-point`) | extern const | `UTCoreTypes.h` | `core_types::MOUNT_POINT` |
| `UTTypeAliasFile` (`com.apple.alias-file`) | extern const | `UTCoreTypes.h` | `core_types::ALIAS_FILE` |
| `UTTypeURLBookmarkData` (`com.apple.bookmark`) | extern const | `UTCoreTypes.h` | `core_types::URL_BOOKMARK_DATA` |
| `UTTypeURL` (`public.url`) | extern const | `UTCoreTypes.h` | `core_types::URL` |
| `UTTypeFileURL` (`public.file-url`) | extern const | `UTCoreTypes.h` | `core_types::FILE_URL` |
| `UTTypeText` (`public.text`) | extern const | `UTCoreTypes.h` | `core_types::TEXT` |
| `UTTypePlainText` (`public.plain-text`) | extern const | `UTCoreTypes.h` | `core_types::PLAIN_TEXT` |
| `UTTypeUTF8PlainText` (`public.utf8-plain-text`) | extern const | `UTCoreTypes.h` | `core_types::UTF8_PLAIN_TEXT` |
| `UTTypeUTF16ExternalPlainText` (`public.utf16-external-plain-text`) | extern const | `UTCoreTypes.h` | `core_types::UTF16_EXTERNAL_PLAIN_TEXT` |
| `UTTypeUTF16PlainText` (`public.utf16-plain-text`) | extern const | `UTCoreTypes.h` | `core_types::UTF16_PLAIN_TEXT` |
| `UTTypeDelimitedText` (`public.delimited-values-text`) | extern const | `UTCoreTypes.h` | `core_types::DELIMITED_TEXT` |
| `UTTypeCommaSeparatedText` (`public.comma-separated-values-text`) | extern const | `UTCoreTypes.h` | `core_types::COMMA_SEPARATED_TEXT` |
| `UTTypeTabSeparatedText` (`public.tab-separated-values-text`) | extern const | `UTCoreTypes.h` | `core_types::TAB_SEPARATED_TEXT` |
| `UTTypeUTF8TabSeparatedText` (`public.utf8-tab-separated-values-text`) | extern const | `UTCoreTypes.h` | `core_types::UTF8_TAB_SEPARATED_TEXT` |
| `UTTypeRTF` (`public.rtf`) | extern const | `UTCoreTypes.h` | `core_types::RTF` |
| `UTTypeHTML` (`public.html`) | extern const | `UTCoreTypes.h` | `core_types::HTML` |
| `UTTypeXML` (`public.xml`) | extern const | `UTCoreTypes.h` | `core_types::XML` |
| `UTTypeYAML` (`public.yaml`) | extern const | `UTCoreTypes.h` | `core_types::YAML` |
| `UTTypeCSS` (`public.css`) | extern const | `UTCoreTypes.h` | `core_types::CSS` |
| `UTTypeSourceCode` (`public.source-code`) | extern const | `UTCoreTypes.h` | `core_types::SOURCE_CODE` |
| `UTTypeAssemblyLanguageSource` (`public.assembly-source`) | extern const | `UTCoreTypes.h` | `core_types::ASSEMBLY_LANGUAGE_SOURCE` |
| `UTTypeCSource` (`public.c-source`) | extern const | `UTCoreTypes.h` | `core_types::C_SOURCE` |
| `UTTypeObjectiveCSource` (`public.objective-c-source`) | extern const | `UTCoreTypes.h` | `core_types::OBJECTIVE_C_SOURCE` |
| `UTTypeSwiftSource` (`public.swift-source`) | extern const | `UTCoreTypes.h` | `core_types::SWIFT_SOURCE` |
| `UTTypeCPlusPlusSource` (`public.c-plus-plus-source`) | extern const | `UTCoreTypes.h` | `core_types::C_PLUS_PLUS_SOURCE` |
| `UTTypeObjectiveCPlusPlusSource` (`public.objective-c-plus-plus-source`) | extern const | `UTCoreTypes.h` | `core_types::OBJECTIVE_C_PLUS_PLUS_SOURCE` |
| `UTTypeCHeader` (`public.c-header`) | extern const | `UTCoreTypes.h` | `core_types::C_HEADER` |
| `UTTypeCPlusPlusHeader` (`public.c-plus-plus-header`) | extern const | `UTCoreTypes.h` | `core_types::C_PLUS_PLUS_HEADER` |
| `UTTypeScript` (`public.script`) | extern const | `UTCoreTypes.h` | `core_types::SCRIPT` |
| `UTTypeAppleScript` (`com.apple.applescript.text`) | extern const | `UTCoreTypes.h` | `core_types::APPLE_SCRIPT_TEXT` |
| `UTTypeOSAScript` (`com.apple.applescript.script`) | extern const | `UTCoreTypes.h` | `core_types::OSA_SCRIPT` |
| `UTTypeOSAScriptBundle` (`com.apple.applescript.script-bundle`) | extern const | `UTCoreTypes.h` | `core_types::OSA_SCRIPT_BUNDLE` |
| `UTTypeJavaScript` (`com.netscape.javascript-source`) | extern const | `UTCoreTypes.h` | `core_types::JAVA_SCRIPT` |
| `UTTypeShellScript` (`public.shell-script`) | extern const | `UTCoreTypes.h` | `core_types::SHELL_SCRIPT` |
| `UTTypePerlScript` (`public.perl-script`) | extern const | `UTCoreTypes.h` | `core_types::PERL_SCRIPT` |
| `UTTypePythonScript` (`public.python-script`) | extern const | `UTCoreTypes.h` | `core_types::PYTHON_SCRIPT` |
| `UTTypeRubyScript` (`public.ruby-script`) | extern const | `UTCoreTypes.h` | `core_types::RUBY_SCRIPT` |
| `UTTypePHPScript` (`public.php-script`) | extern const | `UTCoreTypes.h` | `core_types::PHP_SCRIPT` |
| `UTTypeMakefile` (`public.make-source`) | extern const | `UTCoreTypes.h` | `core_types::MAKEFILE` |
| `UTTypeJSON` (`public.json`) | extern const | `UTCoreTypes.h` | `core_types::JSON` |
| `UTTypePropertyList` (`com.apple.property-list`) | extern const | `UTCoreTypes.h` | `core_types::PROPERTY_LIST` |
| `UTTypeXMLPropertyList` (`com.apple.xml-property-list`) | extern const | `UTCoreTypes.h` | `core_types::XML_PROPERTY_LIST` |
| `UTTypeBinaryPropertyList` (`com.apple.binary-property-list`) | extern const | `UTCoreTypes.h` | `core_types::BINARY_PROPERTY_LIST` |
| `UTTypePDF` (`com.adobe.pdf`) | extern const | `UTCoreTypes.h` | `core_types::PDF` |
| `UTTypeRTFD` (`com.apple.rtfd`) | extern const | `UTCoreTypes.h` | `core_types::RTFD` |
| `UTTypeFlatRTFD` (`com.apple.flat-rtfd`) | extern const | `UTCoreTypes.h` | `core_types::FLAT_RTFD` |
| `UTTypeWebArchive` (`com.apple.webarchive`) | extern const | `UTCoreTypes.h` | `core_types::WEB_ARCHIVE` |
| `UTTypeImage` (`public.image`) | extern const | `UTCoreTypes.h` | `core_types::IMAGE` |
| `UTTypeJPEG` (`public.jpeg`) | extern const | `UTCoreTypes.h` | `core_types::JPEG` |
| `UTTypeTIFF` (`public.tiff`) | extern const | `UTCoreTypes.h` | `core_types::TIFF` |
| `UTTypeGIF` (`com.compuserve.gif`) | extern const | `UTCoreTypes.h` | `core_types::GIF` |
| `UTTypePNG` (`public.png`) | extern const | `UTCoreTypes.h` | `core_types::PNG` |
| `UTTypeICNS` (`com.apple.icns`) | extern const | `UTCoreTypes.h` | `core_types::ICNS` |
| `UTTypeBMP` (`vendor-bmp-identifier`) | extern const | `UTCoreTypes.h` | `core_types::BMP` |
| `UTTypeICO` (`vendor-ico-identifier`) | extern const | `UTCoreTypes.h` | `core_types::ICO` |
| `UTTypeRAWImage` (`public.camera-raw-image`) | extern const | `UTCoreTypes.h` | `core_types::RAW_IMAGE` |
| `UTTypeSVG` (`public.svg-image`) | extern const | `UTCoreTypes.h` | `core_types::SVG` |
| `UTTypeLivePhoto` (`com.apple.live-photo`) | extern const | `UTCoreTypes.h` | `core_types::LIVE_PHOTO` |
| `UTTypeHEIF` (`public.heif`) | extern const | `UTCoreTypes.h` | `core_types::HEIF` |
| `UTTypeHEIC` (`public.heic`) | extern const | `UTCoreTypes.h` | `core_types::HEIC` |
| `UTTypeHEICS` (`public.heics`) | extern const | `UTCoreTypes.h` | `core_types::HEICS` |
| `UTTypeWebP` (`org.webmproject.webp`) | extern const | `UTCoreTypes.h` | `core_types::WEBP` |
| `UTTypeEXR` (`com.ilm.openexr-image`) | extern const | `UTCoreTypes.h` | `core_types::EXR` |
| `UTTypeDNG` (`com.adobe.raw-image`) | extern const | `UTCoreTypes.h` | `core_types::DNG` |
| `UTTypeJPEGXL` (`public.jpeg-xl`) | extern const | `UTCoreTypes.h` | `core_types::JPEG_XL` |
| `UTType3DContent` (`public.3d-content`) | extern const | `UTCoreTypes.h` | `core_types::THREE_D_CONTENT` |
| `UTTypeUSD` (`com.pixar.universal-scene-description`) | extern const | `UTCoreTypes.h` | `core_types::USD` |
| `UTTypeUSDZ` (`com.pixar.universal-scene-description-mobile`) | extern const | `UTCoreTypes.h` | `core_types::USDZ` |
| `UTTypeRealityFile` (`com.apple.reality`) | extern const | `UTCoreTypes.h` | `core_types::REALITY_FILE` |
| `UTTypeSceneKitScene` (`com.apple.scenekit.scene`) | extern const | `UTCoreTypes.h` | `core_types::SCENEKIT_SCENE` |
| `UTTypeARReferenceObject` (`com.apple.arobject`) | extern const | `UTCoreTypes.h` | `core_types::AR_REFERENCE_OBJECT` |
| `UTTypeAudiovisualContent` (`public.audiovisual-content`) | extern const | `UTCoreTypes.h` | `core_types::AUDIOVISUAL_CONTENT` |
| `UTTypeMovie` (`public.movie`) | extern const | `UTCoreTypes.h` | `core_types::MOVIE` |
| `UTTypeVideo` (`public.video`) | extern const | `UTCoreTypes.h` | `core_types::VIDEO` |
| `UTTypeAudio` (`public.audio`) | extern const | `UTCoreTypes.h` | `core_types::AUDIO` |
| `UTTypeQuickTimeMovie` (`com.apple.quicktime-movie`) | extern const | `UTCoreTypes.h` | `core_types::QUICKTIME_MOVIE` |
| `UTTypeMPEG` (`public.mpeg`) | extern const | `UTCoreTypes.h` | `core_types::MPEG` |
| `UTTypeMPEG2Video` (`public.mpeg-2-video`) | extern const | `UTCoreTypes.h` | `core_types::MPEG2_VIDEO` |
| `UTTypeMPEG2TransportStream` (`public.mpeg-2-transport-stream`) | extern const | `UTCoreTypes.h` | `core_types::MPEG2_TRANSPORT_STREAM` |
| `UTTypeMP3` (`public.mp3`) | extern const | `UTCoreTypes.h` | `core_types::MP3` |
| `UTTypeMPEG4Movie` (`public.mpeg-4`) | extern const | `UTCoreTypes.h` | `core_types::MPEG4_MOVIE` |
| `UTTypeMPEG4Audio` (`public.mpeg-4-audio`) | extern const | `UTCoreTypes.h` | `core_types::MPEG4_AUDIO` |
| `UTTypeAppleProtectedMPEG4Audio` (`com.apple.protected-mpeg-4-audio`) | extern const | `UTCoreTypes.h` | `core_types::APPLE_PROTECTED_MPEG4_AUDIO` |
| `UTTypeAppleProtectedMPEG4Video` (`com.apple.protected-mpeg-4-video`) | extern const | `UTCoreTypes.h` | `core_types::APPLE_PROTECTED_MPEG4_VIDEO` |
| `UTTypeAVI` (`public.avi`) | extern const | `UTCoreTypes.h` | `core_types::AVI` |
| `UTTypeAIFF` (`public.aiff-audio`) | extern const | `UTCoreTypes.h` | `core_types::AIFF` |
| `UTTypeWAV` (`vendor-waveform-audio-identifier`) | extern const | `UTCoreTypes.h` | `core_types::WAV` |
| `UTTypeMIDI` (`public.midi-audio`) | extern const | `UTCoreTypes.h` | `core_types::MIDI` |
| `UTTypePlaylist` (`public.playlist`) | extern const | `UTCoreTypes.h` | `core_types::PLAYLIST` |
| `UTTypeM3UPlaylist` (`public.m3u-playlist`) | extern const | `UTCoreTypes.h` | `core_types::M3U_PLAYLIST` |
| `UTTypeFolder` (`public.folder`) | extern const | `UTCoreTypes.h` | `core_types::FOLDER` |
| `UTTypeVolume` (`public.volume`) | extern const | `UTCoreTypes.h` | `core_types::VOLUME` |
| `UTTypePackage` (`com.apple.package`) | extern const | `UTCoreTypes.h` | `core_types::PACKAGE` |
| `UTTypeBundle` (`com.apple.bundle`) | extern const | `UTCoreTypes.h` | `core_types::BUNDLE` |
| `UTTypePluginBundle` (`com.apple.plugin`) | extern const | `UTCoreTypes.h` | `core_types::PLUGIN_BUNDLE` |
| `UTTypeSpotlightImporter` (`com.apple.metadata-importer`) | extern const | `UTCoreTypes.h` | `core_types::SPOTLIGHT_IMPORTER` |
| `UTTypeQuickLookGenerator` (`com.apple.quicklook-generator`) | extern const | `UTCoreTypes.h` | `core_types::QUICK_LOOK_GENERATOR` |
| `UTTypeXPCService` (`com.apple.xpc-service`) | extern const | `UTCoreTypes.h` | `core_types::XPC_SERVICE` |
| `UTTypeFramework` (`com.apple.framework`) | extern const | `UTCoreTypes.h` | `core_types::FRAMEWORK` |
| `UTTypeApplication` (`com.apple.application`) | extern const | `UTCoreTypes.h` | `core_types::APPLICATION` |
| `UTTypeApplicationBundle` (`com.apple.application-bundle`) | extern const | `UTCoreTypes.h` | `core_types::APPLICATION_BUNDLE` |
| `UTTypeApplicationExtension` (`com.apple.application-and-system-extension`) | extern const | `UTCoreTypes.h` | `core_types::APPLICATION_EXTENSION` |
| `UTTypeUnixExecutable` (`public.unix-executable`) | extern const | `UTCoreTypes.h` | `core_types::UNIX_EXECUTABLE` |
| `UTTypeEXE` (`vendor-windows-executable-identifier`) | extern const | `UTCoreTypes.h` | `core_types::EXE` |
| `UTTypeSystemPreferencesPane` (`com.apple.systempreference.prefpane`) | extern const | `UTCoreTypes.h` | `core_types::SYSTEM_PREFERENCES_PANE` |
| `UTTypeArchive` (`public.archive`) | extern const | `UTCoreTypes.h` | `core_types::ARCHIVE` |
| `UTTypeGZIP` (`org.gnu.gnu-zip-archive`) | extern const | `UTCoreTypes.h` | `core_types::GZIP` |
| `UTTypeBZ2` (`public.bzip2-archive`) | extern const | `UTCoreTypes.h` | `core_types::BZ2` |
| `UTTypeZIP` (`public.zip-archive`) | extern const | `UTCoreTypes.h` | `core_types::ZIP` |
| `UTTypeAppleArchive` (`com.apple.archive`) | extern const | `UTCoreTypes.h` | `core_types::APPLE_ARCHIVE` |
| `UTTypeTarArchive` (`public.tar-archive`) | extern const | `UTCoreTypes.h` | `core_types::TAR_ARCHIVE` |
| `UTTypeSpreadsheet` (`public.spreadsheet`) | extern const | `UTCoreTypes.h` | `core_types::SPREADSHEET` |
| `UTTypePresentation` (`public.presentation`) | extern const | `UTCoreTypes.h` | `core_types::PRESENTATION` |
| `UTTypeDatabase` (`public.database`) | extern const | `UTCoreTypes.h` | `core_types::DATABASE` |
| `UTTypeMessage` (`public.message`) | extern const | `UTCoreTypes.h` | `core_types::MESSAGE` |
| `UTTypeContact` (`public.contact`) | extern const | `UTCoreTypes.h` | `core_types::CONTACT` |
| `UTTypeVCard` (`public.vcard`) | extern const | `UTCoreTypes.h` | `core_types::VCARD` |
| `UTTypeToDoItem` (`public.to-do-item`) | extern const | `UTCoreTypes.h` | `core_types::TO_DO_ITEM` |
| `UTTypeCalendarEvent` (`public.calendar-event`) | extern const | `UTCoreTypes.h` | `core_types::CALENDAR_EVENT_ITEM` |
| `UTTypeEmailMessage` (`public.email-message`) | extern const | `UTCoreTypes.h` | `core_types::EMAIL_MESSAGE` |
| `UTTypeInternetLocation` (`com.apple.internet-location`) | extern const | `UTCoreTypes.h` | `core_types::INTERNET_LOCATION` |
| `UTTypeInternetShortcut` (`com.apple.internet-location`) | extern const | `UTCoreTypes.h` | `core_types::INTERNET_SHORTCUT` |
| `UTTypeFont` (`public.font`) | extern const | `UTCoreTypes.h` | `core_types::FONT` |
| `UTTypeBookmark` (`public.bookmark`) | extern const | `UTCoreTypes.h` | `core_types::BOOKMARK` |
| `UTTypePKCS12` (`com.rsa.pkcs-12`) | extern const | `UTCoreTypes.h` | `core_types::PKCS12` |
| `UTTypeX509Certificate` (`public.x509-certificate`) | extern const | `UTCoreTypes.h` | `core_types::X509_CERTIFICATE` |
| `UTTypeEPUB` (`org.idpf.epub-container`) | extern const | `UTCoreTypes.h` | `core_types::EPUB` |
| `UTTypeLog` (`public.log`) | extern const | `UTCoreTypes.h` | `core_types::LOG` |
| `UTTypeAHAP` (`com.apple.haptics.ahap`) | extern const | `UTCoreTypes.h` | `core_types::AHAP` |
| `UTTypeGeoJSON` (`public.geojson`) | extern const | `UTCoreTypes.h` | `core_types::GEOJSON` |
| `UTTypeLinkPresentationMetadata` (`com.apple.linkpresentation.metadata`) | extern const | `UTCoreTypes.h` | `core_types::LINK_PRESENTATION_METADATA` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
(None)

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
(None)
