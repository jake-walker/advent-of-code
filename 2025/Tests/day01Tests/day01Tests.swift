import Foundation
import Testing

@testable import day01

@Test func examplePart1() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day01Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)

    let rotations = parseInput(input: input)
    #expect(part1(rotations) == 3)
}

@Test func examplePart2() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day01Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)

    let rotations = parseInput(input: input)
    #expect(part2(rotations) == 6)
}

@Test func examplePart2Single() async throws {
    var d = Dial()
    let r = Rotation(direction: .Right, count: 1000)
    let v = d.rotate(by: r)
    #expect(v == 10)
    #expect(d.value == 50)
}

@Test func dialLeft99() async throws {
    var d = Dial(value: 99)
    let r = Rotation(direction: .Left, count: 1)
    let v = d.rotate(by: r)
    #expect(v == 0)
    #expect(d.value == 98)
}

@Test func dialLeft0Wraps() async throws {
    var d = Dial(value: 0)
    let r = Rotation(direction: .Left, count: 1)
    let v = d.rotate(by: r)
    #expect(v == 0)
    #expect(d.value == 99)
}

@Test func dialRight98() async throws {
    var d = Dial(value: 98)
    let r = Rotation(direction: .Right, count: 1)
    let v = d.rotate(by: r)
    #expect(v == 0)
    #expect(d.value == 99)
}

@Test func dialRight99Wraps() async throws {
    var d = Dial(value: 99)
    let r = Rotation(direction: .Right, count: 1)
    let v = d.rotate(by: r)
    #expect(v == 1)
    #expect(d.value == 0)
}
