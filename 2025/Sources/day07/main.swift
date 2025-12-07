import Foundation

public enum ManifoldPart: Equatable {
    case Empty
    case Splitter
    case Beam(weight: Int)
}

public struct TachyonManifold {
    let start: (Int, Int)
    var map: [[ManifoldPart]]

    public func toString(withWeights: Bool = false) -> String {
        return map.map({ row in
            row.map({ cell in
                switch cell {
                case .Beam(weight: let w):
                    if withWeights {
                        return "\(w)"
                    } else {
                        return "|"
                    }
                case .Empty:
                    return "."
                case .Splitter:
                    return "^"
                }
            }).joined()
        }).joined(separator: "\n")
    }
}

extension TachyonManifold: Equatable {
    public static func == (lhs: TachyonManifold, rhs: TachyonManifold) -> Bool {
        return lhs.start.0 == rhs.start.0 && lhs.start.1 == rhs.start.1 && lhs.map == rhs.map
    }
}

public func parseInput(input: String) -> TachyonManifold {
    var start: (Int, Int)? = nil
    var map: [[ManifoldPart]] = []

    for (y, line) in input.split(whereSeparator: \.isNewline).enumerated() {
        var mapRow: [ManifoldPart] = []

        for (x, char) in String(line).enumerated() {
            switch char {
            case "S":
                start = (x, y)
                mapRow.append(.Empty)
            case ".":
                mapRow.append(.Empty)
            case "^":
                mapRow.append(.Splitter)
            default:
                fatalError("Unknown input character '\(char)'!")
            }
        }

        map.append(mapRow)
    }

    guard let start else {
        fatalError("No start position found in input")
    }

    return TachyonManifold(start: start, map: map)
}

public func propagateBeam(_ input: TachyonManifold) -> (
    TachyonManifold, Int
) {
    var splitCount = 0
    var m = input.map

    m[input.start.1][input.start.0] = .Beam(weight: 1)

    for r in 1..<m.count {
        for c in 0..<m[r].count {
            // if the cell above is a beam
            if case .Beam(let weight) = m[r - 1][c] {
                if m[r][c] == .Splitter {
                    splitCount += 1

                    // if the current space is a splitter, copy the beam to either side of the current cell
                    // ...checking for bounds
                    if c > 0 {
                        if case .Beam(let currentWeight) = m[r][c - 1] {
                            // if the current space is a beam, add the weight of the new beam and current beam
                            m[r][c - 1] = .Beam(weight: currentWeight + weight)
                        } else {
                            // otherwise, the new beam keeps the same weight value
                            m[r][c - 1] = .Beam(weight: weight)
                        }
                    }
                    if c < m[r].count {
                        if case .Beam(let currentWeight) = m[r][c + 1] {
                            m[r][c + 1] = .Beam(weight: currentWeight + weight)
                        } else {
                            m[r][c + 1] = .Beam(weight: weight)
                        }
                    }
                } else {
                    // if the current space is empty, copy the beam to this cell
                    // ...with same weight
                    if case .Beam(let currentWeight) = m[r][c] {
                        m[r][c] = .Beam(weight: currentWeight + weight)
                    } else {
                        m[r][c] = .Beam(weight: weight)
                    }
                }
            }
        }
    }

    return (TachyonManifold(start: input.start, map: m), splitCount)
}

public func sumWeights(_ input: TachyonManifold) -> Int {
    let finalRow = input.map.last!
    let weights = finalRow.flatMap({
        if case .Beam(let w) = $0 {
            return w
        } else {
            return nil
        }
    })

    return weights.reduce(0, +)
}

@main
struct Day07 {
    static func main() throws {
        let fileUrl = URL(fileURLWithPath: "Sources/day07/input.txt")
        let input = try String(contentsOf: fileUrl, encoding: .utf8)
        let parsed = parseInput(input: input)

        let (result, splitCount) = propagateBeam(parsed)
        let weightSum = sumWeights(result)

        print("Part 1: \(splitCount)")
        print("Part 2: \(weightSum)")
    }
}
