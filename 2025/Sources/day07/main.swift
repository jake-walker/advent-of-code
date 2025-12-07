import Foundation

public enum ManifoldPart {
    case Empty
    case Splitter
    case Beam
}

public struct TachyonManifold {
    let start: (Int, Int)
    var map: [[ManifoldPart]]

    public func toString() -> String {
        return map.map({ row in
            row.map({ cell in
                switch cell {
                case .Beam:
                    return "|"
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
                mapRow.append(.Beam)
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

public func propagateBeam(_ input: TachyonManifold) -> (TachyonManifold, Int) {
    var splitCount = 0
    var m = input.map

    for r in 1..<m.count {
        for c in 0..<m[r].count {
            // if the space above is not a beam, we don't care
            if m[r - 1][c] != .Beam {
                continue
            }

            if m[r][c] == .Empty {
                // if the current space is empty, copy the beam to this cell
                m[r][c] = .Beam
            } else if m[r][c] == .Splitter {
                splitCount += 1

                // if the current space is a splitter, copy the beam to either side of the current cell
                // ...checking for bounds
                if c > 0 {
                    m[r][c - 1] = .Beam
                }
                if c < m[r].count {
                    m[r][c + 1] = .Beam
                }
            }
        }
    }

    return (TachyonManifold(start: input.start, map: m), splitCount)
}

@main
struct Day07 {
    static func main() throws {
        let fileUrl = URL(fileURLWithPath: "Sources/day07/input.txt")
        let input = try String(contentsOf: fileUrl, encoding: .utf8)
        let parsed = parseInput(input: input)

        print("Part 1: \(propagateBeam(parsed).1)")
    }
}
