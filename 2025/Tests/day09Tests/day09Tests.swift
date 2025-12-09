import Foundation
import Testing
import utils

@testable import day09

@Test func testParse() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day09Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)
    let parsed = parseInput(input: input)

    #expect(
        parsed == [
            Point(7, 1),
            Point(11, 1),
            Point(11, 7),
            Point(9, 7),
            Point(9, 5),
            Point(2, 5),
            Point(2, 3),
            Point(7, 3),
        ]
    )
}

@Test func testCalculateArea() async throws {
    #expect(calculateArea(Point(2, 5), Point(11, 1)) == 50)
}

@Test func testPart1() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day09Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)
    let parsed = parseInput(input: input)

    #expect(largestArea(points: parsed) == 50)
}

// @Test func testPart2() async throws {
//     let fileUrl = URL(fileURLWithPath: "Tests/day08Tests/example.txt")
//     let input = try String(contentsOf: fileUrl, encoding: .utf8)
//     let parsed = parseInput(input: input)

//     let (_, lastMerged) = processPoints(parsed, iterations: nil)

//     #expect(lastMerged.0 == Point3(216, 146, 977))
//     #expect(lastMerged.1 == Point3(117, 168, 530))
// }
