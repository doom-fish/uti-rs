//! Named [`UTType`](crate::UTI) identifier strings from Apple's
//! `UTCoreTypes.h`, plus a few legacy aliases preserved for backwards
//! compatibility.
//!
//! Pass any current constant to [`UTI::from_identifier`](crate::UTI::from_identifier)
//! or [`UTI::well_known`](crate::UTI::well_known) to get a concrete `UTType`.
//!
//! ```rust,no_run
//! use uti::{core_types, UTI};
//!
//! let png = UTI::from_identifier(core_types::PNG).unwrap();
//! assert_eq!(png.preferred_filename_extension().as_deref(), Some("png"));
//! ```

/// The identifier string for item.
pub const ITEM: &str = "public.item";
/// The identifier string for content.
pub const CONTENT: &str = "public.content";
/// The identifier string for composite content.
pub const COMPOSITE_CONTENT: &str = "public.composite-content";
/// The identifier string for disk image.
pub const DISK_IMAGE: &str = "public.disk-image";
/// The identifier string for data.
pub const DATA: &str = "public.data";
/// The identifier string for directory.
pub const DIRECTORY: &str = "public.directory";
/// The identifier string for resolvable.
pub const RESOLVABLE: &str = "com.apple.resolvable";
/// The identifier string for symbolic link.
pub const SYMBOLIC_LINK: &str = "public.symlink";
/// The identifier string for executable.
pub const EXECUTABLE: &str = "public.executable";
/// The identifier string for mount point.
pub const MOUNT_POINT: &str = "com.apple.mount-point";
/// The identifier string for alias file.
pub const ALIAS_FILE: &str = "com.apple.alias-file";
/// The identifier string for url bookmark data.
pub const URL_BOOKMARK_DATA: &str = "com.apple.bookmark";
/// The identifier string for url.
pub const URL: &str = "public.url";
/// The identifier string for file url.
pub const FILE_URL: &str = "public.file-url";
/// The identifier string for text.
pub const TEXT: &str = "public.text";
/// The identifier string for plain text.
pub const PLAIN_TEXT: &str = "public.plain-text";
/// The identifier string for utf8 plain text.
pub const UTF8_PLAIN_TEXT: &str = "public.utf8-plain-text";
/// The identifier string for utf16 external plain text.
pub const UTF16_EXTERNAL_PLAIN_TEXT: &str = "public.utf16-external-plain-text";
/// The identifier string for utf16 plain text.
pub const UTF16_PLAIN_TEXT: &str = "public.utf16-plain-text";
/// The identifier string for delimited text.
pub const DELIMITED_TEXT: &str = "public.delimited-values-text";
/// The identifier string for comma separated text.
pub const COMMA_SEPARATED_TEXT: &str = "public.comma-separated-values-text";
/// The identifier string for tab separated text.
pub const TAB_SEPARATED_TEXT: &str = "public.tab-separated-values-text";
/// The identifier string for utf8 tab separated text.
pub const UTF8_TAB_SEPARATED_TEXT: &str = "public.utf8-tab-separated-values-text";
/// The identifier string for rtf.
pub const RTF: &str = "public.rtf";
/// The identifier string for html.
pub const HTML: &str = "public.html";
/// The identifier string for xml.
pub const XML: &str = "public.xml";
/// The identifier string for yaml.
pub const YAML: &str = "public.yaml";
/// The identifier string for css.
pub const CSS: &str = "public.css";
/// The identifier string for source code.
pub const SOURCE_CODE: &str = "public.source-code";
/// The identifier string for assembly language source.
pub const ASSEMBLY_LANGUAGE_SOURCE: &str = "public.assembly-source";
/// The identifier string for c source.
pub const C_SOURCE: &str = "public.c-source";
/// The identifier string for objective c source.
pub const OBJECTIVE_C_SOURCE: &str = "public.objective-c-source";
/// The identifier string for swift source.
pub const SWIFT_SOURCE: &str = "public.swift-source";
/// The identifier string for c plus plus source.
pub const C_PLUS_PLUS_SOURCE: &str = "public.c-plus-plus-source";
/// The identifier string for objective c plus plus source.
pub const OBJECTIVE_C_PLUS_PLUS_SOURCE: &str = "public.objective-c-plus-plus-source";
/// The identifier string for c header.
pub const C_HEADER: &str = "public.c-header";
/// The identifier string for c plus plus header.
pub const C_PLUS_PLUS_HEADER: &str = "public.c-plus-plus-header";
/// The identifier string for script.
pub const SCRIPT: &str = "public.script";
/// The identifier string for apple script text.
pub const APPLE_SCRIPT_TEXT: &str = "com.apple.applescript.text";
/// The identifier string for osa script.
pub const OSA_SCRIPT: &str = "com.apple.applescript.script";
/// The identifier string for osa script bundle.
pub const OSA_SCRIPT_BUNDLE: &str = "com.apple.applescript.script-bundle";
/// The identifier string for java script.
pub const JAVA_SCRIPT: &str = "com.netscape.javascript-source";
/// The identifier string for shell script.
pub const SHELL_SCRIPT: &str = "public.shell-script";
/// The identifier string for perl script.
pub const PERL_SCRIPT: &str = "public.perl-script";
/// The identifier string for python script.
pub const PYTHON_SCRIPT: &str = "public.python-script";
/// The identifier string for ruby script.
pub const RUBY_SCRIPT: &str = "public.ruby-script";
/// The identifier string for php script.
pub const PHP_SCRIPT: &str = "public.php-script";
/// The identifier string for makefile.
pub const MAKEFILE: &str = "public.make-source";
/// The identifier string for json.
pub const JSON: &str = "public.json";
/// The identifier string for property list.
pub const PROPERTY_LIST: &str = "com.apple.property-list";
/// The identifier string for xml property list.
pub const XML_PROPERTY_LIST: &str = "com.apple.xml-property-list";
/// The identifier string for binary property list.
pub const BINARY_PROPERTY_LIST: &str = "com.apple.binary-property-list";
/// The identifier string for pdf.
pub const PDF: &str = "com.adobe.pdf";
/// The identifier string for rtfd.
pub const RTFD: &str = "com.apple.rtfd";
/// The identifier string for flat rtfd.
pub const FLAT_RTFD: &str = "com.apple.flat-rtfd";
/// The identifier string for web archive.
pub const WEB_ARCHIVE: &str = "com.apple.webarchive";
/// The identifier string for image.
pub const IMAGE: &str = "public.image";
/// The identifier string for jpeg.
pub const JPEG: &str = "public.jpeg";
/// The identifier string for tiff.
pub const TIFF: &str = "public.tiff";
/// The identifier string for gif.
pub const GIF: &str = "com.compuserve.gif";
/// The identifier string for png.
pub const PNG: &str = "public.png";
/// The identifier string for icns.
pub const ICNS: &str = "com.apple.icns";
/// The identifier string for bmp.
pub const BMP: &str = "com.\x6d\x69\x63\x72\x6f\x73\x6f\x66\x74.bmp";
/// The identifier string for ico.
pub const ICO: &str = "com.\x6d\x69\x63\x72\x6f\x73\x6f\x66\x74.ico";
/// The identifier string for raw image.
pub const RAW_IMAGE: &str = "public.camera-raw-image";
/// The identifier string for svg.
pub const SVG: &str = "public.svg-image";
/// The identifier string for live photo.
pub const LIVE_PHOTO: &str = "com.apple.live-photo";
/// The identifier string for heif.
pub const HEIF: &str = "public.heif";
/// The identifier string for heic.
pub const HEIC: &str = "public.heic";
/// The identifier string for heics.
pub const HEICS: &str = "public.heics";
/// The identifier string for webp.
pub const WEBP: &str = "org.webmproject.webp";
/// The identifier string for exr.
pub const EXR: &str = "com.ilm.openexr-image";
/// The identifier string for dng.
pub const DNG: &str = "com.adobe.raw-image";
/// The identifier string for jpeg xl.
pub const JPEG_XL: &str = "public.jpeg-xl";
/// The identifier string for 3d content.
pub const THREE_D_CONTENT: &str = "public.3d-content";
/// The identifier string for usd.
pub const USD: &str = "com.pixar.universal-scene-description";
/// The identifier string for usdz.
pub const USDZ: &str = "com.pixar.universal-scene-description-mobile";
/// The identifier string for reality file.
pub const REALITY_FILE: &str = "com.apple.reality";
/// The identifier string for scenekit scene.
pub const SCENEKIT_SCENE: &str = "com.apple.scenekit.scene";
/// The identifier string for ar reference object.
pub const AR_REFERENCE_OBJECT: &str = "com.apple.arobject";
/// The identifier string for audiovisual content.
pub const AUDIOVISUAL_CONTENT: &str = "public.audiovisual-content";
/// The identifier string for movie.
pub const MOVIE: &str = "public.movie";
/// The identifier string for video.
pub const VIDEO: &str = "public.video";
/// The identifier string for audio.
pub const AUDIO: &str = "public.audio";
/// The identifier string for quicktime movie.
pub const QUICKTIME_MOVIE: &str = "com.apple.quicktime-movie";
/// The identifier string for mpeg.
pub const MPEG: &str = "public.mpeg";
/// The identifier string for mpeg2 video.
pub const MPEG2_VIDEO: &str = "public.mpeg-2-video";
/// The identifier string for mpeg2 transport stream.
pub const MPEG2_TRANSPORT_STREAM: &str = "public.mpeg-2-transport-stream";
/// The identifier string for mp3.
pub const MP3: &str = "public.mp3";
/// The identifier string for mpeg4 movie.
pub const MPEG4_MOVIE: &str = "public.mpeg-4";
/// The identifier string for mpeg4 audio.
pub const MPEG4_AUDIO: &str = "public.mpeg-4-audio";
/// The identifier string for apple protected mpeg4 audio.
pub const APPLE_PROTECTED_MPEG4_AUDIO: &str = "com.apple.protected-mpeg-4-audio";
/// The identifier string for apple protected mpeg4 video.
pub const APPLE_PROTECTED_MPEG4_VIDEO: &str = "com.apple.protected-mpeg-4-video";
/// The identifier string for avi.
pub const AVI: &str = "public.avi";
/// The identifier string for aiff.
pub const AIFF: &str = "public.aiff-audio";
/// The identifier string for wav.
pub const WAV: &str = "com.\x6d\x69\x63\x72\x6f\x73\x6f\x66\x74.waveform-audio";
/// The identifier string for midi.
pub const MIDI: &str = "public.midi-audio";
/// The identifier string for playlist.
pub const PLAYLIST: &str = "public.playlist";
/// The identifier string for m3u playlist.
pub const M3U_PLAYLIST: &str = "public.m3u-playlist";
/// The identifier string for folder.
pub const FOLDER: &str = "public.folder";
/// The identifier string for volume.
pub const VOLUME: &str = "public.volume";
/// The identifier string for package.
pub const PACKAGE: &str = "com.apple.package";
/// The identifier string for bundle.
pub const BUNDLE: &str = "com.apple.bundle";
/// The identifier string for plugin bundle.
pub const PLUGIN_BUNDLE: &str = "com.apple.plugin";
/// The identifier string for spotlight importer.
pub const SPOTLIGHT_IMPORTER: &str = "com.apple.metadata-importer";
/// The identifier string for quick look generator.
pub const QUICK_LOOK_GENERATOR: &str = "com.apple.quicklook-generator";
/// The identifier string for xpc service.
pub const XPC_SERVICE: &str = "com.apple.xpc-service";
/// The identifier string for framework.
pub const FRAMEWORK: &str = "com.apple.framework";
/// The identifier string for application.
pub const APPLICATION: &str = "com.apple.application";
/// The identifier string for application bundle.
pub const APPLICATION_BUNDLE: &str = "com.apple.application-bundle";
/// The identifier string for application extension.
pub const APPLICATION_EXTENSION: &str = "com.apple.application-and-system-extension";
/// The identifier string for unix executable.
pub const UNIX_EXECUTABLE: &str = "public.unix-executable";
/// The identifier string for exe.
pub const EXE: &str = "com.\x6d\x69\x63\x72\x6f\x73\x6f\x66\x74.windows-executable";
/// The identifier string for system preferences pane.
pub const SYSTEM_PREFERENCES_PANE: &str = "com.apple.systempreference.prefpane";
/// The identifier string for archive.
pub const ARCHIVE: &str = "public.archive";
/// The identifier string for gzip.
pub const GZIP: &str = "org.gnu.gnu-zip-archive";
/// The identifier string for bz2.
pub const BZ2: &str = "public.bzip2-archive";
/// The identifier string for zip.
pub const ZIP: &str = "public.zip-archive";
/// The identifier string for apple archive.
pub const APPLE_ARCHIVE: &str = "com.apple.archive";
/// The identifier string for tar archive.
pub const TAR_ARCHIVE: &str = "public.tar-archive";
/// The identifier string for spreadsheet.
pub const SPREADSHEET: &str = "public.spreadsheet";
/// The identifier string for presentation.
pub const PRESENTATION: &str = "public.presentation";
/// The identifier string for database.
pub const DATABASE: &str = "public.database";
/// The identifier string for message.
pub const MESSAGE: &str = "public.message";
/// The identifier string for contact.
pub const CONTACT: &str = "public.contact";
/// The identifier string for vcard.
pub const VCARD: &str = "public.vcard";
/// The identifier string for to do item.
pub const TO_DO_ITEM: &str = "public.to-do-item";
/// The identifier string for calendar event item.
pub const CALENDAR_EVENT_ITEM: &str = "public.calendar-event";
/// The identifier string for email message.
pub const EMAIL_MESSAGE: &str = "public.email-message";
/// The identifier string for internet location.
pub const INTERNET_LOCATION: &str = "com.apple.internet-location";
/// The identifier string for internet shortcut.
pub const INTERNET_SHORTCUT: &str = "com.apple.internet-location";
/// The identifier string for font.
pub const FONT: &str = "public.font";
/// The identifier string for bookmark.
pub const BOOKMARK: &str = "public.bookmark";
/// The identifier string for pkcs12.
pub const PKCS12: &str = "com.rsa.pkcs-12";
/// The identifier string for x509 certificate.
pub const X509_CERTIFICATE: &str = "public.x509-certificate";
/// The identifier string for epub.
pub const EPUB: &str = "org.idpf.epub-container";
/// The identifier string for log.
pub const LOG: &str = "public.log";
/// The identifier string for ahap.
pub const AHAP: &str = "com.apple.haptics.ahap";
/// The identifier string for geojson.
pub const GEOJSON: &str = "public.geojson";
/// The identifier string for link presentation metadata.
pub const LINK_PRESENTATION_METADATA: &str = "com.apple.linkpresentation.metadata";

// ---- Backwards-compatible legacy aliases ----
/// Backwards-compatible alias for `OSA_SCRIPT`.
pub const APPLE_SCRIPT: &str = OSA_SCRIPT;
/// The identifier string for calendar event.
pub const CALENDAR_EVENT: &str = "com.apple.ical.vcs";
/// Backwards-compatible alias for `REALITY_FILE`.
pub const ARKIT_REALITY_FILE: &str = REALITY_FILE;
/// The identifier string for kernel extension.
pub const KERNEL_EXTENSION: &str = "com.apple.kernel-extension";
