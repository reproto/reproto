import PackageDescription

let package = Package(
    name: "swift",
    targets: [
        Target(
            name: "swift",
            dependencies: ["Models"]
        ),
        Target(
            name: "Models",
            dependencies: []
        ),
    ]
)
