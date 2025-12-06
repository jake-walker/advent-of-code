import Foundation

private func transposeArray<T>(_ array: [[T]]) -> [[T]] {
    let rowCount = array.count
    let colCount = array[0].count
    var transposed: [[T]] = Array(
        repeating: Array(repeating: array[0][0], count: rowCount), count: colCount)

    for i in 0..<rowCount {
        for j in 0..<colCount {
            transposed[j][i] = array[i][j]
        }
    }

    return transposed
}

public func parseInput(input: String) -> [[String]] {
    return transposeArray(
        input.split(whereSeparator: \.isNewline)
            .map({ line in
                line.split(whereSeparator: \.isWhitespace)
                    .map({ String($0) })
                    .filter({ !String($0).trimmingCharacters(in: .whitespaces).isEmpty })
            })
    )
}

public func calculateAll(_ input: [[String]]) -> [Int] {
    var results: [Int] = []

    for item in input {
        let operation = item.last!
        let numbers = item.dropLast().map({ Int($0)! })

        switch operation {
        case "+":
            results.append(numbers.reduce(0, +))
        case "*":
            results.append(numbers.reduce(1, *))
        default:
            fatalError("Unexpected operation '\(operation)'")
        }
    }

    return results
}

@main
struct Day06 {
    static func main() throws {
        let fileUrl = URL(fileURLWithPath: "Sources/day06/input.txt")
        let input = try String(contentsOf: fileUrl, encoding: .utf8)
        let parsed = parseInput(input: input)

        print("Part 1: \(calculateAll(parsed).reduce(0, +))")
    }
}
