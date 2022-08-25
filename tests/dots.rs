use witness::{solve, Edge, EdgeDirection, IntersectionOrEdge, Pos, Puzzle};

#[test]
fn simple_sat() {
    let puzzle = Puzzle {
        width: 1,
        height: 1,
        sources: vec![IntersectionOrEdge::Intersection(Pos { x: 0, y: 0 })],
        exits: vec![IntersectionOrEdge::Intersection(Pos { x: 1, y: 1 })],
        broken: vec![],
        dots: vec![IntersectionOrEdge::Edge(Edge {
            pos: Pos { x: 0, y: 0 },
            dir: EdgeDirection::Vertical,
        })],
    };
    assert_eq!(
        solve(&puzzle),
        Some(vec![
            IntersectionOrEdge::Intersection(Pos { x: 0, y: 0 }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 0, y: 0 },
                dir: EdgeDirection::Vertical
            }),
            IntersectionOrEdge::Intersection(Pos { x: 0, y: 1 }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 0, y: 1 },
                dir: EdgeDirection::Horizontal
            }),
            IntersectionOrEdge::Intersection(Pos { x: 1, y: 1 })
        ])
    );
}

#[test]
fn dead_end() {
    let puzzle = Puzzle {
        width: 1,
        height: 1,
        sources: vec![IntersectionOrEdge::Intersection(Pos { x: 0, y: 0 })],
        exits: vec![IntersectionOrEdge::Intersection(Pos { x: 1, y: 1 })],
        broken: vec![IntersectionOrEdge::Edge(Edge {
            pos: Pos { x: 1, y: 0 },
            dir: EdgeDirection::Vertical,
        })],
        dots: vec![IntersectionOrEdge::Intersection(Pos { x: 1, y: 0 })],
    };
    assert_eq!(solve(&puzzle), None);
}

#[test]
fn no_loops() {
    let puzzle = Puzzle {
        width: 1,
        height: 2,
        sources: vec![IntersectionOrEdge::Intersection(Pos { x: 0, y: 0 })],
        exits: vec![IntersectionOrEdge::Intersection(Pos { x: 1, y: 0 })],
        broken: vec![
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 0, y: 0 },
                dir: EdgeDirection::Vertical,
            }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 1, y: 0 },
                dir: EdgeDirection::Vertical,
            }),
        ],
        dots: vec![IntersectionOrEdge::Intersection(Pos { x: 0, y: 2 })],
    };
    assert_eq!(solve(&puzzle), None);
}
