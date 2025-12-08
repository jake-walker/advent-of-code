public struct DisjointSet {
    private var parent: [Int]
    private var size: [Int]

    init(count n: Int) {
        parent = Array(0..<n)
        size = Array(repeating: 1, count: n)
    }

    mutating func find(_ x: Int) -> Int {
        var x = x
        while parent[x] != x {
            parent[x] = parent[parent[x]]
            x = parent[x]
        }
        return x
    }

    mutating func union(_ a: Int, _ b: Int) -> Bool {
        var ra = find(a)
        var rb = find(b)
        if ra == rb { return false }
        if size[ra] < size[rb] { swap(&ra, &rb) }
        parent[rb] = ra
        size[ra] += size[rb]
        return true
    }

    mutating func componentSize(of x: Int) -> Int {
        let r = find(x)
        return size[r]
    }

    mutating func allComponentSizes() -> [Int] {
        var rootSeen = Set<Int>()
        var results: [Int] = []
        for i in 0..<parent.count {
            let r = find(i)
            if !rootSeen.contains(r) {
                rootSeen.insert(r)
                results.append(size[r])
            }
        }
        return results
    }
}
