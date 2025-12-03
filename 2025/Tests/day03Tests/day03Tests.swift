import Foundation
import Testing

@testable import day03

@Test func parseExample() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day03Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)

    #expect(
        parseInput(input: input) == [
            [9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            [8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            [2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            [8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ])
}

@Test func examplePart1() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day03Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)
    let parsed = parseInput(input: input)

    #expect(part1(ratings: parsed) == 357)
}

@Test func examplePart2() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day03Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)
    let parsed = parseInput(input: input)

    #expect(part1(ratings: parsed, count: 12) == 3_121_910_778_619)
}
