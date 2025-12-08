import Foundation

public struct Point3: Equatable, Hashable {
    let x: Int
    let y: Int
    let z: Int

    public init(_ x: Int, _ y: Int, _ z: Int) {
        self.x = x
        self.y = y
        self.z = z
    }

    public func squareDistance(_ c: Point3) -> Int64 {
        let dx = Int64(self.x) - Int64(c.x)
        let dy = Int64(self.y) - Int64(c.y)
        let dz = Int64(self.z) - Int64(c.z)
        return dx * dx + dy * dy + dz * dz
    }

    public func euclideanDistance(_ c: Point3) -> Double {
        return sqrt(Double(self.squareDistance(c)))
    }
}

public typealias Point3Pair = (Int64, Int, Int)

public func parseInput(input: String) -> [Point3] {
    return input.split(whereSeparator: \.isNewline)
        .filter({ !$0.isEmpty })
        .map({ line in
            let parts = line.split(separator: ",")
            return Point3(Int(parts[0])!, Int(parts[1])!, Int(parts[2])!)
        })
}

func pointPairs(_ points: [Point3]) -> [Point3Pair] {
    var combos: [Point3Pair] = []

    for i in 0..<points.count - 1 {
        for j in (i + 1)..<points.count {
            let d = points[i].squareDistance(points[j])
            combos.append((d, i, j))
        }
    }

    combos.sort(by: { a, b in a.0 < b.0 })

    return combos
}

public func processPoints(_ points: [Point3], iterations: Int = 1000) -> [Int] {
    let pairs = pointPairs(points)
    var ds = DisjointSet(count: points.count)
    let limit = min(iterations, pairs.count)
    var processed = 0
    var idx = 0
    while processed < limit && idx < pairs.count {
        let (_, i, j) = pairs[idx]
        _ = ds.union(i, j)
        processed += 1
        idx += 1
    }

    var sizes = ds.allComponentSizes()
    sizes.sort(by: >)
    return sizes
}

@main
struct Day08 {
    static func main() throws {
        let fileUrl = URL(fileURLWithPath: "Sources/day08/input.txt")
        let input = try String(contentsOf: fileUrl, encoding: .utf8)
        let parsed = parseInput(input: input)

        let sizes = processPoints(parsed)

        print("Part 1: \(sizes.prefix(3).reduce(1, *))")
        // print("Part 2: \(weightSum)")
    }
}
