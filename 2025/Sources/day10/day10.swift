import Algorithms
import Foundation

public struct Machine: Equatable, Sendable {
    let indicators: Int
    let buttons: [Int]
    let joltage: [Int]

    public init(indicators: [Bool], buttons: [[Bool]], joltage: [Int]) {
        self.indicators = convertBoolArrayToMask(indicators)
        self.buttons = buttons.map({ convertBoolArrayToMask($0) })
        self.joltage = joltage
    }

    public init(indicators: Int, buttons: [Int], joltage: [Int]) {
        self.indicators = indicators
        self.buttons = buttons
        self.joltage = joltage
    }
}

private func convertBoolArrayToMask(_ a: [Bool]) -> Int {
    var mask = 0
    for (i, v) in a.enumerated() {
        if v {
            mask |= (1 << i)
        }
    }
    return mask
}

public func parseInput(input: String) -> [Machine] {
    var output: [Machine] = []
    let lineRegex = /\[([.#]+)\] \(([\(\) ,\d]+)\) \{([\d,]+)\}/

    for line in input.split(whereSeparator: \.isNewline) {
        if let match = try? lineRegex.wholeMatch(in: line) {
            var indicators: [Bool] = []
            var buttons: [[Bool]] = []
            var joltage: [Int] = []

            for char in match.output.1 {
                switch char {
                case ".":
                    indicators.append(false)
                case "#":
                    indicators.append(true)
                default:
                    fatalError("Unexpected indicator light character \"\(char)\"")
                }
            }

            for group in match.output.2.split(whereSeparator: \.isWhitespace) {
                var buttonGroup: [Bool] = Array(repeating: false, count: indicators.count)

                for index in group.trimmingCharacters(in: .init(charactersIn: "()")).split(
                    separator: ",")
                {
                    buttonGroup[Int(index)!] = true
                }

                buttons.append(buttonGroup)
            }

            for joltageValue in match.output.3.split(separator: ",") {
                joltage.append(Int(joltageValue)!)
            }

            output.append(Machine(indicators: indicators, buttons: buttons, joltage: joltage))
        }
    }

    return output
}

public func minPresses(for machine: Machine) -> (Int, [Int])? {
    for k in 0...machine.buttons.count {
        for combo in machine.buttons.combinations(ofCount: k) {
            var state = 0

            for x in combo {
                state ^= x
            }

            if state == machine.indicators {
                return (k, combo)
            }
        }
    }

    return nil
}

public func minPressesAll(machines: [Machine]) -> Int {
    var sum = 0

    for (i, machine) in machines.enumerated() {
        guard let (i, _) = minPresses(for: machine) else {
            fatalError("Failed to calculate combination for machine #\(i)")
        }

        sum += i
    }

    return sum
}

@main
struct Day10 {
    static func main() throws {
        let fileUrl = URL(fileURLWithPath: "Sources/day10/input.txt")
        let input = try String(contentsOf: fileUrl, encoding: .utf8)
        let parsed = parseInput(input: input)

        print("Part 1: \(minPressesAll(machines: parsed))")
    }
}
