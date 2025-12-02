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

public func part2(ranges: [ClosedRange<Int>]) -> Int {
    var sum = 0

    for range in ranges {
        for n in range {
            let nStr = String(n)
            let maxPartLength = Int(floor(Double(nStr.count) / 2.0))

            // if there's not enough characters to make any combinations, go to the next number
            if maxPartLength < 1 {
                continue
            }

            for l in 1...maxPartLength {
                // if the string length is not divisible by the part length, go to the next part length
                if nStr.count % l != 0 {
                    continue
                }

                // split the string into equal parts of `l` length
                let parts = stride(from: 0, to: nStr.count, by: l).map {
                    let start = nStr.index(nStr.startIndex, offsetBy: $0)
                    let end =
                        nStr.index(start, offsetBy: l, limitedBy: nStr.endIndex) ?? nStr.endIndex
                    return String(nStr[start..<end])
                }

                // if all the parts are equal, add to the sum
                if parts.allSatisfy({ $0 == parts.first }) {
                    sum += n
                    break
                }
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
        print("Part 2: \(part2(ranges: parsed))")
    }
}
