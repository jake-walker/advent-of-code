import Foundation
import Testing

@testable import day06

@Test func testParse() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day06Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)
    let parsed = parseInput(input: input)

    #expect(
        parsed == [
            ["123", "45", "6", "*"],
            ["328", "64", "98", "+"],
            ["51", "387", "215", "*"],
            ["64", "23", "314", "+"],
        ]
    )
}

@Test func examplePart1() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day06Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)
    let parsed = parseInput(input: input)

    #expect(calculateAll(parsed) == [33210, 490, 4_243_455, 401])
}

@Test func examplePart2() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day06Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)

    #expect(calculateP2(input) == 3_263_827)
}
