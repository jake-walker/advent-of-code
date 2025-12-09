import Foundation
import utils

public func parseInput(input: String) -> [Point] {
    return input.split(whereSeparator: \.isNewline)
        .map({ line in
            let parts = line.split(separator: ",")
            return Point(Int(parts[0])!, Int(parts[1])!)
        })
}

public func calculateArea(_ a: Point, _ b: Point) -> Int {
    return (abs(a.x - b.x) + 1) * (abs(a.y - b.y) + 1)
}

public func largestArea(points: [Point]) -> Int {
    var largestArea = 0

    for i in 0..<points.count {
        for j in (i + 1)..<points.count {
            let area = calculateArea(points[i], points[j])

            if area > largestArea {
                largestArea = area
            }
        }
    }

    return largestArea
}

@main
struct Day09 {
    static func main() throws {
        let fileUrl = URL(fileURLWithPath: "Sources/day09/input.txt")
        let input = try String(contentsOf: fileUrl, encoding: .utf8)
        let parsed = parseInput(input: input)

        print("Part 1: \(largestArea(points: parsed))")
        // print("Part 2: \(lastMerged.0.x * lastMerged.1.x)")
    }
}
