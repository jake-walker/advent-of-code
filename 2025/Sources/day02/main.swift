import Foundation

public func parseInput(input: String) -> [ClosedRange<Int>] {
    var output: [ClosedRange<Int>] = []

    let lines = input.trimmingCharacters(in: .whitespacesAndNewlines).split(separator: ",")
    for line in lines {
        let parts = line.split(separator: "-")

        if parts.count != 2 {
            fatalError("Invalid input range")
        }

        output.append(Int(parts[0])!...Int(parts[1])!)
    }

    return output
}

public func part1(ranges: [ClosedRange<Int>]) -> Int {
    var sum = 0

    for range in ranges {
        for n in range {
            let nStr = String(n)
            if nStr.count % 2 != 0 {
                continue
            }

            let index = nStr.index(nStr.startIndex, offsetBy: nStr.count / 2)
            let p1 = nStr[..<index]
            let p2 = nStr[index...]
            if p1 == p2 {
                sum += n
            }
        }
    }

    return sum
}

@main
struct Day02 {
    static func main() throws {
        let fileUrl = URL(fileURLWithPath: "Sources/day02/input.txt")
        let input = try String(contentsOf: fileUrl, encoding: .utf8)
        let parsed = parseInput(input: input)

        print("Part 1: \(part1(ranges: parsed))")
    }
}
