import Foundation

public func parseInput(input: String) -> [[Bool]] {
    return input.split(whereSeparator: \.isNewline)
        .map({ line in
            line.map({ $0 == "@" })
        })
}

public func part1(map: [[Bool]], maxCount: Int = 4) -> Int {
    var accessibleCount = 0

    for (y, row) in map.enumerated() {
        for (x, cell) in row.enumerated() {
            if !cell {
                continue
            }

            var surroundingRolls = 0

            for i in -1...1 {
                for j in -1...1 {
                    if i == 0 && j == 0 {
                        continue
                    }

                    let y1 = y + i
                    let x1 = x + j

                    if y1 >= 0 && y1 < map.count && x1 >= 0 && x1 < row.count && map[y1][x1] {
                        surroundingRolls += 1
                    }

                    if surroundingRolls >= maxCount {
                        break
                    }
                }

                if surroundingRolls >= maxCount {
                    break
                }
            }

            if surroundingRolls < maxCount {
                accessibleCount += 1
            }
        }
    }

    return accessibleCount
}

@main
struct Day04 {
    static func main() throws {
        let fileUrl = URL(fileURLWithPath: "Sources/day04/input.txt")
        let input = try String(contentsOf: fileUrl, encoding: .utf8)
        let parsed = parseInput(input: input)

        print("Part 1: \(part1(map: parsed))")
    }
}
