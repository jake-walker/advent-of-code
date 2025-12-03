import Foundation

public func parseInput(input: String) -> [[Int]] {
    return input.split(whereSeparator: \.isNewline)
        .map({ line in
            line.map({ Int(String($0))! })
        })
}

public func part1(ratings: [[Int]], count: Int = 2) -> Int {
    var sum = 0

    for bank in ratings {
        var result = 0
        var startIndex = bank.startIndex

        for i in 0..<count {
            let endIndex = bank.endIndex - (count - i)

            var currentLargestIndex: Int? = nil

            // search to the right finding the highest value
            for j in startIndex...endIndex {
                if currentLargestIndex == nil || bank[j] > bank[currentLargestIndex!] {
                    currentLargestIndex = j
                }
            }

            if let currentLargestIndex {
                let multiplier = i != count - 1 ? Int(pow(10.0, Double(count - i) - 1.0)) : 1
                result += bank[currentLargestIndex] * multiplier
                startIndex = currentLargestIndex + 1
            } else {
                fatalError("Failed to find largest digit")
            }
        }

        sum += result
    }

    return sum
}

@main
struct Day03 {
    static func main() throws {
        let fileUrl = URL(fileURLWithPath: "Sources/day03/input.txt")
        let input = try String(contentsOf: fileUrl, encoding: .utf8)
        let parsed = parseInput(input: input)

        print("Part 1: \(part1(ratings: parsed))")
        print("Part 2: \(part1(ratings: parsed, count: 12))")
    }
}
