import Foundation

public enum Direction: Character {
    case Left = "L"
    case Right = "R"
}

public struct Rotation {
    let direction: Direction
    let count: Int
}

public func parseInput(input: String) -> [Rotation] {
    var output: [Rotation] = []

    let lines = input.split(whereSeparator: \.isNewline)
    for line in lines {
        let direction = Direction(rawValue: line.first!)!
        let count = Int(line.dropFirst())!

        output.append(Rotation(direction: direction, count: count))
    }

    return output
}

public struct Dial {
    var value: Int = 50

    private func wrapped(_ x: Int, mod: Int) -> Int {
        let r = x % mod
        return r >= 0 ? r : r + mod
    }

    private func floorDiv(_ a: Int, by b: Int) -> Int {
        let (q, r) = a.quotientAndRemainder(dividingBy: b)
        if (r != 0) && ((a < 0) != (b < 0)) {
            return q - 1
        } else {
            return q
        }
    }

    mutating func rotate(by rotation: Rotation) -> Int {
        let mod = 100
        let old = self.value
        let v = rotation.count

        if rotation.direction == .Right {
            let rawNew = old + v
            self.value = wrapped(rawNew, mod: mod)
            return floorDiv(rawNew, by: mod) - floorDiv(old, by: mod)
        } else {
            let rawNew = old - v
            self.value = wrapped(rawNew, mod: mod)
            return floorDiv(-rawNew, by: mod) - floorDiv(-old, by: mod)
        }
    }
}

public func part1(_ rotations: [Rotation]) -> Int {
    var zeroCount = 0
    var dial = Dial()

    for rotation in rotations {
        let _ = dial.rotate(by: rotation)
        if dial.value == 0 {
            zeroCount += 1
        }
    }

    return zeroCount
}

public func part2(_ rotations: [Rotation]) -> Int {
    var zeroCount = 0
    var dial = Dial()

    for rotation in rotations {
        zeroCount += dial.rotate(by: rotation)
    }

    return zeroCount
}

@main
struct Day01 {
    static func main() throws {
        let fileUrl = URL(fileURLWithPath: "Sources/day01/input.txt")
        let input = try String(contentsOf: fileUrl, encoding: .utf8)
        let parsed = parseInput(input: input)

        print("Part 1: \(part1(parsed))")
        print("Part 2: \(part2(parsed))")
    }
}
