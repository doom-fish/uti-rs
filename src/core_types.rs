//! Named [`UTType`](crate::UTI) identifier strings for the most-used
//! Apple types — `UTCoreTypes.h` equivalents.
//!
//! Pass any of these to [`UTI::well_known`](crate::UTI::well_known) or
//! [`UTI::from_identifier`](crate::UTI::from_identifier) to get a
//! concrete `UTType`.
//!
//! ```rust,no_run
//! use uti::{core_types, UTI};
//!
//! let png = UTI::from_identifier(core_types::PNG).unwrap();
//! assert_eq!(png.preferred_filename_extension().as_deref(), Some("png"));
//! ```

// ---- Generic / abstract ----
pub const ITEM: &str = "public.item";
pub const CONTENT: &str = "public.content";
pub const COMPOSITE_CONTENT: &str = "public.composite-content";
pub const DATA: &str = "public.data";
pub const DIRECTORY: &str = "public.directory";
pub const FOLDER: &str = "public.folder";
pub const PACKAGE: &str = "com.apple.package";
pub const BUNDLE: &str = "com.apple.bundle";
pub const EXECUTABLE: &str = "public.executable";
pub const SYMBOLIC_LINK: &str = "public.symlink";
pub const DISK_IMAGE: &str = "public.disk-image";

// ---- URLs / files ----
pub const URL: &str = "public.url";
pub const FILE_URL: &str = "public.file-url";

// ---- Text ----
pub const TEXT: &str = "public.text";
pub const PLAIN_TEXT: &str = "public.plain-text";
pub const UTF8_PLAIN_TEXT: &str = "public.utf8-plain-text";
pub const UTF16_PLAIN_TEXT: &str = "public.utf16-plain-text";
pub const RTF: &str = "public.rtf";
pub const HTML: &str = "public.html";
pub const XML: &str = "public.xml";
pub const YAML: &str = "public.yaml";
pub const JSON: &str = "public.json";
pub const CSS: &str = "public.css";

// ---- Source code ----
pub const SOURCE_CODE: &str = "public.source-code";
pub const C_SOURCE: &str = "public.c-source";
pub const OBJECTIVE_C_SOURCE: &str = "public.objective-c-source";
pub const C_PLUS_PLUS_SOURCE: &str = "public.c-plus-plus-source";
pub const SWIFT_SOURCE: &str = "public.swift-source";
pub const SCRIPT: &str = "public.script";
pub const APPLE_SCRIPT: &str = "com.apple.applescript.script";
pub const SHELL_SCRIPT: &str = "public.shell-script";
pub const PYTHON_SCRIPT: &str = "public.python-script";
pub const PERL_SCRIPT: &str = "public.perl-script";
pub const RUBY_SCRIPT: &str = "public.ruby-script";
pub const PHP_SCRIPT: &str = "public.php-script";

// ---- Images ----
pub const IMAGE: &str = "public.image";
pub const JPEG: &str = "public.jpeg";
pub const TIFF: &str = "public.tiff";
pub const GIF: &str = "com.compuserve.gif";
pub const PNG: &str = "public.png";
pub const ICNS: &str = "com.apple.icns";
pub const BMP: &str = "com.microsoft.bmp";
pub const ICO: &str = "com.microsoft.ico";
pub const RAW_IMAGE: &str = "public.camera-raw-image";
pub const SVG: &str = "public.svg-image";
pub const LIVE_PHOTO: &str = "com.apple.live-photo";
pub const HEIF: &str = "public.heif";
pub const HEIC: &str = "public.heic";
pub const WEBP: &str = "org.webmproject.webp";

// ---- Video / audio / movies ----
pub const AUDIOVISUAL_CONTENT: &str = "public.audiovisual-content";
pub const MOVIE: &str = "public.movie";
pub const VIDEO: &str = "public.video";
pub const AUDIO: &str = "public.audio";
pub const QUICKTIME_MOVIE: &str = "com.apple.quicktime-movie";
pub const MPEG: &str = "public.mpeg";
pub const MPEG2_VIDEO: &str = "public.mpeg-2-video";
pub const MPEG4_MOVIE: &str = "public.mpeg-4";
pub const MPEG4_AUDIO: &str = "public.mpeg-4-audio";
pub const AVI: &str = "public.avi";
pub const APPLE_PROTECTED_MPEG4_AUDIO: &str = "com.apple.protected-mpeg-4-audio";
pub const MP3: &str = "public.mp3";
pub const WAV: &str = "com.microsoft.waveform-audio";
pub const AIFF: &str = "public.aiff-audio";
pub const APPLE_PROTECTED_MPEG4_VIDEO: &str = "com.apple.protected-mpeg-4-video";

// ---- 3D ----
pub const USDZ: &str = "com.pixar.universal-scene-description-mobile";
pub const SCENEKIT_SCENE: &str = "com.apple.scenekit.scene";
pub const ARKIT_REALITY_FILE: &str = "com.apple.reality";

// ---- Documents ----
pub const PDF: &str = "com.adobe.pdf";
pub const RTFD: &str = "com.apple.rtfd";
pub const FLAT_RTFD: &str = "com.apple.flat-rtfd";

// ---- Archives ----
pub const ARCHIVE: &str = "public.archive";
pub const GZIP: &str = "org.gnu.gnu-zip-archive";
pub const BZ2: &str = "public.bzip2-archive";
pub const ZIP: &str = "public.zip-archive";
pub const APPLE_ARCHIVE: &str = "com.apple.archive";

// ---- Calendar / contacts ----
pub const CALENDAR_EVENT: &str = "com.apple.ical.vcs";
pub const VCARD: &str = "public.vcard";

// ---- macOS bundles ----
pub const APPLICATION: &str = "com.apple.application";
pub const APPLICATION_BUNDLE: &str = "com.apple.application-bundle";
pub const FRAMEWORK: &str = "com.apple.framework";
pub const PLUGIN_BUNDLE: &str = "com.apple.plugin";
pub const KERNEL_EXTENSION: &str = "com.apple.kernel-extension";
