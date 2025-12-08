import Foundation
import Testing

@testable import day08

func nearlyEqual(_ a: Double, _ b: Double, tol: Double = 1e-3) -> Bool {
    return (a - b) < tol
}

@Test func testParse() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day08Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)
    let parsed = parseInput(input: input)

    #expect(
        parsed == [
            Point3(162, 817, 812),
            Point3(57, 618, 57),
            Point3(906, 360, 560),
            Point3(592, 479, 940),
            Point3(352, 342, 300),
            Point3(466, 668, 158),
            Point3(542, 29, 236),
            Point3(431, 825, 988),
            Point3(739, 650, 466),
            Point3(52, 470, 668),
            Point3(216, 146, 977),
            Point3(819, 987, 18),
            Point3(117, 168, 530),
            Point3(805, 96, 715),
            Point3(346, 949, 466),
            Point3(970, 615, 88),
            Point3(941, 993, 340),
            Point3(862, 61, 35),
            Point3(984, 92, 344),
            Point3(425, 690, 689),
        ]
    )
}

@Test func testEuclideanDistance() async throws {
    let a = Point3(162, 817, 812)
    let b = Point3(425, 690, 689)

    #expect(nearlyEqual(a.euclideanDistance(b), 316.902))
}

@Test func testPart1() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day08Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)
    let parsed = parseInput(input: input)

    let (sizes, _) = processPoints(parsed, iterations: 10)

    #expect(sizes.prefix(3) == [5, 4, 2])
}

@Test func testPart2() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day08Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)
    let parsed = parseInput(input: input)

    let (_, lastMerged) = processPoints(parsed, iterations: nil)

    #expect(lastMerged.0 == Point3(216, 146, 977))
    #expect(lastMerged.1 == Point3(117, 168, 530))
}
