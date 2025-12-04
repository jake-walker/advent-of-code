import Foundation
import Testing

@testable import day04

@Test func testParse() async throws {
    #expect(
        parseInput(input: "..@@\n@@@.") == [
            [false, false, true, true],
            [true, true, true, false],
        ])
}

@Test func examplePart1() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day04Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)
    let parsed = parseInput(input: input)

    #expect(part1(map: parsed) == 13)
}
