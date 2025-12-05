import Foundation
import Testing

@testable import day05

@Test func testParse() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day05Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)
    let parsed = parseInput(input: input)

    #expect(
        parsed
            == Ingredients(
                fresh: [
                    3...5,
                    10...14,
                    16...20,
                    12...18,
                ],
                available: [1, 5, 8, 11, 17, 32]
            )
    )
}

@Test func examplePart1() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day05Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)
    let parsed = parseInput(input: input)

    #expect(part1(ingredients: parsed) == 3)
}

@Test func examplePart2() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day05Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)
    let parsed = parseInput(input: input)

    #expect(part2(ingredients: parsed) == 14)
}
