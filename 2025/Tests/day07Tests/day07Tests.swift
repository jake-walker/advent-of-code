import Foundation
import Testing

@testable import day07

@Test func testParse() async throws {
    let parsed = parseInput(input: "..S..\n.^.^.\n.....")

    #expect(
        parsed
            == TachyonManifold(
                start: (2, 0),
                map: [
                    [.Empty, .Empty, .Beam, .Empty, .Empty],
                    [.Empty, .Splitter, .Empty, .Splitter, .Empty],
                    [.Empty, .Empty, .Empty, .Empty, .Empty],
                ]
            )
    )
}

@Test func examplePropagate() async throws {
    let fileUrl = URL(fileURLWithPath: "Tests/day07Tests/example.txt")
    let input = try String(contentsOf: fileUrl, encoding: .utf8)
    let parsed = parseInput(input: input)

    let (propagated, splitCount) = propagateBeam(parsed)

    #expect(
        propagated.toString() == """
            .......|.......
            .......|.......
            ......|^|......
            ......|.|......
            .....|^|^|.....
            .....|.|.|.....
            ....|^|^|^|....
            ....|.|.|.|....
            ...|^|^|||^|...
            ...|.|.|||.|...
            ..|^|^|||^|^|..
            ..|.|.|||.|.|..
            .|^|||^||.||^|.
            .|.|||.||.||.|.
            |^|^|^|^|^|||^|
            |.|.|.|.|.|||.|
            """)
    #expect(splitCount == 21)
}
