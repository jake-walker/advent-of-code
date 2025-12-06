// swift-tools-version: 6.2
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "day01",
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .executable(name: "day01", targets: ["day01"]),
        .executable(name: "day02", targets: ["day02"]),
        .executable(name: "day03", targets: ["day03"]),
        .executable(name: "day04", targets: ["day04"]),
        .executable(name: "day05", targets: ["day05"]),
        .executable(name: "day06", targets: ["day06"]),
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .executableTarget(name: "day01"),
        .testTarget(name: "day01Tests", dependencies: ["day01"]),
        .executableTarget(name: "day02"),
        .testTarget(name: "day02Tests", dependencies: ["day02"]),
        .executableTarget(name: "day03"),
        .testTarget(name: "day03Tests", dependencies: ["day03"]),
        .executableTarget(name: "day04"),
        .testTarget(name: "day04Tests", dependencies: ["day04"]),
        .executableTarget(name: "day05"),
        .testTarget(name: "day05Tests", dependencies: ["day05"]),
        .executableTarget(name: "day06"),
        .testTarget(name: "day06Tests", dependencies: ["day06"]),
    ]
)
