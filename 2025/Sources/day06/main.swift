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

public func calculateP2(_ input: String) -> Int {
    var sum = 0
    var operation: Character? = nil
    var cursor = 0
    let lines = input.split(whereSeparator: \.isNewline).map({ String($0) })
    var values: [Int] = []
    let limit = lines.map({ $0.count }).max()!

    while cursor < limit {
        let chars = lines.map({ line in
            cursor >= line.count ? " " : line[line.index(line.startIndex, offsetBy: cursor)]
        })

        // if the operation is an empty string, we're at the start of a new calculation
        if operation == nil {
            operation = chars.last!

            if operation?.isWhitespace ?? true {
                fatalError("Failed to get current operation (cursor=\(cursor))")
            }
        }

        // next calculation - set operation to an empty string
        if chars.allSatisfy({ $0 == " " }) {
            // finalize last
            switch operation {
            case "+":
                sum += values.reduce(0, +)
            case "*":
                sum += values.reduce(1, *)
            default:
                fatalError("Unexpected operation '\(String(describing: operation))'")
            }

            values = []
            operation = nil
            cursor += 1
            continue
        }

        var val = 0
        let digits = chars.dropLast().filter({ !$0.isWhitespace })
        for (i, digit) in digits.enumerated() {
            if digit.isWhitespace {
                continue
            }

            let pos = digits.count - i - 1
            let base = pos <= 0 ? 1 : Int(pow(10.0, Double(pos)))
            val += Int(String(digit))! * base
        }

        values.insert(val, at: 0)
        cursor += 1
    }

    // calculate final values
    switch operation {
    case "+":
        sum += values.reduce(0, +)
    case "*":
        sum += values.reduce(1, *)
    default:
        fatalError("Unexpected operation '\(String(describing: operation))'")
    }

    return sum
}

@main
struct Day06 {
    static func main() throws {
        let fileUrl = URL(fileURLWithPath: "Sources/day06/input.txt")
        let input = try String(contentsOf: fileUrl, encoding: .utf8)
        let parsed = parseInput(input: input)

        print("Part 1: \(calculateAll(parsed).reduce(0, +))")
        print("Part 2: \(calculateP2(input))")
    }
}
