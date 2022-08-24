use witness::{solve, Edge, EdgeDirection, IntersectionOrEdge, Pos, Puzzle};

#[test]
fn minimal_sat() {
    let puzzle = Puzzle {
        width: 0,
        height: 0,
        sources: vec![IntersectionOrEdge::Intersection(Pos { x: 0, y: 0 })],
        exits: vec![IntersectionOrEdge::Intersection(Pos { x: 0, y: 0 })],
        broken: vec![],
        dots: vec![],
    };
    assert_eq!(
        solve(&puzzle),
        Some(vec![IntersectionOrEdge::Intersection(Pos { x: 0, y: 0 })])
    );
}

#[test]
fn minimal_unsat() {
    let puzzle = Puzzle {
        width: 0,
        height: 1,
        sources: vec![IntersectionOrEdge::Intersection(Pos { x: 0, y: 0 })],
        exits: vec![IntersectionOrEdge::Intersection(Pos { x: 0, y: 1 })],
        broken: vec![IntersectionOrEdge::Edge(Edge {
            pos: Pos { x: 0, y: 0 },
            dir: EdgeDirection::Vertical,
        })],
        dots: vec![],
    };
    assert_eq!(solve(&puzzle), None);
}

#[test]
fn simple_sat() {
    let puzzle = Puzzle {
        width: 1,
        height: 1,
        sources: vec![IntersectionOrEdge::Intersection(Pos { x: 0, y: 0 })],
        exits: vec![IntersectionOrEdge::Intersection(Pos { x: 1, y: 1 })],
        broken: vec![IntersectionOrEdge::Edge(Edge {
            pos: Pos { x: 0, y: 0 },
            dir: EdgeDirection::Vertical,
        })],
        dots: vec![],
    };
    assert_eq!(
        solve(&puzzle),
        Some(vec![
            IntersectionOrEdge::Intersection(Pos { x: 0, y: 0 }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 0, y: 0 },
                dir: EdgeDirection::Horizontal
            }),
            IntersectionOrEdge::Intersection(Pos { x: 1, y: 0 }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 1, y: 0 },
                dir: EdgeDirection::Vertical
            }),
            IntersectionOrEdge::Intersection(Pos { x: 1, y: 1 })
        ])
    );
}

#[test]
fn moving_upwards() {
    let puzzle = Puzzle {
        width: 1,
        height: 1,
        sources: vec![IntersectionOrEdge::Intersection(Pos { x: 1, y: 1 })],
        exits: vec![IntersectionOrEdge::Intersection(Pos { x: 0, y: 0 })],
        broken: vec![IntersectionOrEdge::Edge(Edge {
            pos: Pos { x: 0, y: 0 },
            dir: EdgeDirection::Horizontal,
        })],
        dots: vec![],
    };
    assert_eq!(
        solve(&puzzle),
        Some(vec![
            IntersectionOrEdge::Intersection(Pos { x: 1, y: 1 }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 0, y: 1 },
                dir: EdgeDirection::Horizontal
            }),
            IntersectionOrEdge::Intersection(Pos { x: 0, y: 1 }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 0, y: 0 },
                dir: EdgeDirection::Vertical
            }),
            IntersectionOrEdge::Intersection(Pos { x: 0, y: 0 })
        ])
    );
}

#[test]
fn simple_unsat() {
    let puzzle = Puzzle {
        width: 1,
        height: 1,
        sources: vec![IntersectionOrEdge::Intersection(Pos { x: 0, y: 0 })],
        exits: vec![IntersectionOrEdge::Intersection(Pos { x: 1, y: 1 })],
        broken: vec![
            IntersectionOrEdge::Intersection(Pos { x: 0, y: 1 }),
            IntersectionOrEdge::Intersection(Pos { x: 1, y: 0 }),
        ],
        dots: vec![],
    };
    assert_eq!(solve(&puzzle), None);
}

#[test]
fn sat_with_edges() {
    let puzzle = Puzzle {
        width: 2,
        height: 2,
        sources: vec![IntersectionOrEdge::Edge(Edge {
            pos: Pos { x: 0, y: 0 },
            dir: EdgeDirection::Horizontal,
        })],
        exits: vec![IntersectionOrEdge::Edge(Edge {
            pos: Pos { x: 2, y: 1 },
            dir: EdgeDirection::Vertical,
        })],
        broken: vec![
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 0, y: 0 },
                dir: EdgeDirection::Vertical,
            }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 1, y: 1 },
                dir: EdgeDirection::Vertical,
            }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 2, y: 0 },
                dir: EdgeDirection::Vertical,
            }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 1, y: 1 },
                dir: EdgeDirection::Horizontal,
            }),
        ],
        dots: vec![],
    };
    assert!(matches!(solve(&puzzle), Some(_)));
}

#[test]
fn unsat_with_edges() {
    let puzzle = Puzzle {
        width: 2,
        height: 2,
        sources: vec![IntersectionOrEdge::Edge(Edge {
            pos: Pos { x: 0, y: 0 },
            dir: EdgeDirection::Horizontal,
        })],
        exits: vec![IntersectionOrEdge::Edge(Edge {
            pos: Pos { x: 2, y: 1 },
            dir: EdgeDirection::Vertical,
        })],
        broken: vec![
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 0, y: 0 },
                dir: EdgeDirection::Vertical,
            }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 1, y: 1 },
                dir: EdgeDirection::Vertical,
            }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 2, y: 0 },
                dir: EdgeDirection::Vertical,
            }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 1, y: 1 },
                dir: EdgeDirection::Horizontal,
            }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 0, y: 2 },
                dir: EdgeDirection::Horizontal,
            }),
        ],
        dots: vec![],
    };
    assert_eq!(solve(&puzzle), None);
}

#[test]
fn no_source() {
    let puzzle = Puzzle {
        width: 1,
        height: 1,
        sources: vec![],
        exits: vec![
            IntersectionOrEdge::Intersection(Pos { x: 0, y: 0 }),
            IntersectionOrEdge::Intersection(Pos { x: 1, y: 1 }),
        ],
        broken: vec![],
        dots: vec![],
    };
    assert_eq!(solve(&puzzle), None);
}

#[test]
fn no_exit() {
    let puzzle = Puzzle {
        width: 1,
        height: 1,
        sources: vec![
            IntersectionOrEdge::Intersection(Pos { x: 0, y: 0 }),
            IntersectionOrEdge::Intersection(Pos { x: 1, y: 1 }),
        ],
        exits: vec![],
        broken: vec![],
        dots: vec![],
    };
    assert_eq!(solve(&puzzle), None);
}

#[test]
fn multiple_sources() {
    let puzzle = Puzzle {
        width: 1,
        height: 2,
        sources: vec![
            IntersectionOrEdge::Intersection(Pos { x: 0, y: 0 }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 0, y: 1 },
                dir: EdgeDirection::Vertical,
            }),
        ],
        exits: vec![IntersectionOrEdge::Intersection(Pos { x: 1, y: 1 })],
        broken: vec![
            IntersectionOrEdge::Intersection(Pos { x: 0, y: 1 }),
            IntersectionOrEdge::Intersection(Pos { x: 1, y: 0 }),
        ],
        dots: vec![],
    };
    assert!(matches!(solve(&puzzle), Some(_)));
}

#[test]
fn multiple_exits() {
    let puzzle = Puzzle {
        width: 1,
        height: 2,
        sources: vec![IntersectionOrEdge::Intersection(Pos { x: 0, y: 0 })],
        exits: vec![
            IntersectionOrEdge::Intersection(Pos { x: 1, y: 1 }),
            IntersectionOrEdge::Intersection(Pos { x: 1, y: 2 }),
        ],
        broken: vec![
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 1, y: 1 },
                dir: EdgeDirection::Vertical,
            }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 0, y: 2 },
                dir: EdgeDirection::Horizontal,
            }),
        ],
        dots: vec![],
    };
    assert!(matches!(solve(&puzzle), Some(_)));
}
