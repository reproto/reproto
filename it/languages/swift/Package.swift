// swift-tools-version:5.2
import PackageDescription

let package = Package(
    name: "reproto-swift-it",
    targets: [
        .target(name: "ReprotoTest", dependencies: ["Models"]),
        .target(name: "Models", dependencies: []),
    ]
)
