// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "UTIBridge",
    platforms: [.macOS(.v13)],
    products: [
        .library(name: "UTIBridge", type: .static, targets: ["UTIBridge"]),
    ],
    targets: [
        .target(name: "UTIBridge", path: "Sources/UTIBridge", publicHeadersPath: "include"),
    ]
)
