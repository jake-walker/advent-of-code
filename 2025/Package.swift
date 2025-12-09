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
        .executable(name: "day07", targets: ["day07"]),
        .executable(name: "day08", targets: ["day08"]),
        .executable(name: "day09", targets: ["day09"]),
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(name: "utils"),
        .executableTarget(name: "day01", exclude: ["input.txt"]),
        .testTarget(name: "day01Tests", dependencies: ["day01"], resources: [.copy("example.txt")]),
        .executableTarget(name: "day02", exclude: ["input.txt"]),
        .testTarget(name: "day02Tests", dependencies: ["day02"], resources: [.copy("example.txt")]),
        .executableTarget(name: "day03", exclude: ["input.txt"]),
        .testTarget(name: "day03Tests", dependencies: ["day03"], resources: [.copy("example.txt")]),
        .executableTarget(name: "day04", exclude: ["input.txt"]),
        .testTarget(name: "day04Tests", dependencies: ["day04"], resources: [.copy("example.txt")]),
        .executableTarget(name: "day05", exclude: ["input.txt"]),
        .testTarget(name: "day05Tests", dependencies: ["day05"], resources: [.copy("example.txt")]),
        .executableTarget(name: "day06", exclude: ["input.txt"]),
        .testTarget(name: "day06Tests", dependencies: ["day06"], resources: [.copy("example.txt")]),
        .executableTarget(name: "day07", exclude: ["input.txt"]),
        .testTarget(name: "day07Tests", dependencies: ["day07"], resources: [.copy("example.txt")]),
        .executableTarget(name: "day08", exclude: ["input.txt"]),
        .testTarget(name: "day08Tests", dependencies: ["day08"], resources: [.copy("example.txt")]),
        .executableTarget(name: "day09", dependencies: ["utils"], exclude: ["input.txt"]),
        .testTarget(
            name: "day09Tests", dependencies: ["day09", "utils"], resources: [.copy("example.txt")]),
    ]
)
