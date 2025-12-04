import Foundation

public func parseInput(input: String) -> [[Bool]] {
    return input.split(whereSeparator: \.isNewline)
        .map({ line in
            line.map({ $0 == "@" })
        })
}

public func getAccessible(map: [[Bool]]) -> [(Int, Int)] {
    let maxCount = 4
    var accessible: [(Int, Int)] = []

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
                accessible.append((y, x))
            }
        }
    }

    return accessible
}

public func part2(map: [[Bool]]) -> Int {
    var map = map
    var removed = 0

    while true {
        let accessible = getAccessible(map: map)

        if accessible.isEmpty {
            return removed
        }

        for (y, x) in accessible {
            map[y][x] = false
            removed += 1
        }
    }
}

@main
struct Day04 {
    static func main() throws {
        let fileUrl = URL(fileURLWithPath: "Sources/day04/input.txt")
        let input = try String(contentsOf: fileUrl, encoding: .utf8)
        let parsed = parseInput(input: input)

        print("Part 1: \(getAccessible(map: parsed).count)")
        print("Part 2: \(part2(map: parsed))")
    }
}
