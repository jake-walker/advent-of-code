import Foundation
import Testing

@testable import day02

@Test func examplePart1() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day02Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)

    let ranges = parseInput(input: input)
    #expect(part1(ranges: ranges) == 1_227_775_554)
}
