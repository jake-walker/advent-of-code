import Foundation

public struct Ingredients {
    var fresh: [ClosedRange<Int>]
    var available: [Int]
}

extension Ingredients: Equatable {
    public static func == (lhs: Ingredients, rhs: Ingredients) -> Bool {
        return lhs.fresh == rhs.fresh && lhs.available == rhs.available
    }
}

public func parseInput(input: String) -> Ingredients {
    var output = Ingredients(fresh: [], available: [])

    let sections = input.components(separatedBy: "\n\n")

    guard sections.count == 2,
        let freshSection = sections.first,
        let availableSection = sections.last
    else {
        fatalError("Invalid input")
    }

    for line in freshSection.split(whereSeparator: \.isNewline) {
        let parts = line.split(separator: "-")

        guard let lowerStr = parts.first,
            let upperStr = parts.last,
            let lower = Int(lowerStr),
            let upper = Int(upperStr)
        else {
            fatalError("Invalid input")
        }

        output.fresh.append(lower...upper)
    }

    for line in availableSection.split(whereSeparator: \.isNewline) {
        guard let n = Int(line) else {
            fatalError("Invalid input")
        }

        output.available.append(n)
    }

    return output
}

public func part1(ingredients: Ingredients) -> Int {
    var count = 0

    for item in ingredients.available {
        let isFresh = ingredients.fresh.contains(where: { $0.contains(item) })

        if isFresh {
            count += 1
        }
    }

    return count
}

public func part2(ingredients: Ingredients) -> Int {
    let sortedRanges = ingredients.fresh.sorted(by: { $0.lowerBound <= $1.lowerBound })

    var merged: [ClosedRange<Int>] = []
    var currentLower = sortedRanges.first!.lowerBound
    var currentUpper = sortedRanges.first!.upperBound

    for range in sortedRanges.dropFirst() {
        if range.lowerBound <= currentUpper + 1 {
            currentUpper = max(currentUpper, range.upperBound)
        } else {
            merged.append(currentLower...currentUpper)
            currentLower = range.lowerBound
            currentUpper = range.upperBound
        }
    }

    merged.append(currentLower...currentUpper)

    return merged.map({ $0.count }).reduce(0, +)
}

@main
struct Day05 {
    static func main() throws {
        let fileUrl = URL(fileURLWithPath: "Sources/day05/input.txt")
        let input = try String(contentsOf: fileUrl, encoding: .utf8)
        let parsed = parseInput(input: input)

        print("Part 1: \(part1(ingredients: parsed))")
        print("Part 2: \(part2(ingredients: parsed))")
    }
}
