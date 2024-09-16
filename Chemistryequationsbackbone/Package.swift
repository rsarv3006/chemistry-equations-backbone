// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
// Swift Package: Chemistryequationsbackbone

import PackageDescription;

let package = Package(
    name: "Chemistryequationsbackbone",
    platforms: [
        .iOS(.v13),
        .macOS(.v10_15)
    ],
    products: [
        .library(
            name: "Chemistryequationsbackbone",
            targets: ["Chemistryequationsbackbone"]
        )
    ],
    dependencies: [ ],
    targets: [
        .binaryTarget(name: "RustFramework", path: "./RustFramework.xcframework"),
        .target(
            name: "Chemistryequationsbackbone",
            dependencies: [
                .target(name: "RustFramework")
            ]
        ),
    ]
)