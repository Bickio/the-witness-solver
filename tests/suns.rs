use witness::{
    solve, Colour, ColouredSymbol, Edge, EdgeDirection, IntersectionOrEdge, Pos, Puzzle,
};

#[test]
fn two_suns_sat() {
    let puzzle = Puzzle {
        width: 1,
        height: 2,
        sources: vec![IntersectionOrEdge::Intersection(Pos { x: 0, y: 1 })],
        exits: vec![IntersectionOrEdge::Intersection(Pos { x: 1, y: 1 })],
        broken: vec![],
        dots: vec![],
        squares: vec![
            ColouredSymbol {
                pos: Pos { x: 0, y: 0 },
                colour: Colour::Orange,
            },
            ColouredSymbol {
                pos: Pos { x: 0, y: 1 },
                colour: Colour::Orange,
            },
        ],
        suns: vec![],
    };
    assert!(matches!(solve(&puzzle), Some(_)));
}

#[test]
fn two_suns_unsat() {
    let puzzle = Puzzle {
        width: 1,
        height: 2,
        sources: vec![IntersectionOrEdge::Intersection(Pos { x: 0, y: 2 })],
        exits: vec![IntersectionOrEdge::Intersection(Pos { x: 1, y: 0 })],
        broken: vec![],
        dots: vec![IntersectionOrEdge::Edge(Edge {
            pos: Pos { x: 0, y: 1 },
            dir: EdgeDirection::Horizontal,
        })],
        squares: vec![],
        suns: vec![
            ColouredSymbol {
                pos: Pos { x: 0, y: 0 },
                colour: Colour::Orange,
            },
            ColouredSymbol {
                pos: Pos { x: 0, y: 1 },
                colour: Colour::Orange,
            },
        ],
    };
    let result = solve(&puzzle);
    println!("{:#?}", result);
    assert!(matches!(result, None));
}

#[test]
fn suns_and_squares_simple() {
    let puzzle = Puzzle {
        width: 2,
        height: 2,
        sources: vec![IntersectionOrEdge::Intersection(Pos { x: 1, y: 2 })],
        exits: vec![IntersectionOrEdge::Intersection(Pos { x: 1, y: 0 })],
        broken: vec![],
        dots: vec![],
        squares: vec![
            ColouredSymbol {
                pos: Pos { x: 0, y: 0 },
                colour: Colour::White,
            },
            ColouredSymbol {
                pos: Pos { x: 1, y: 0 },
                colour: Colour::Black,
            },
        ],
        suns: vec![ColouredSymbol {
            pos: Pos { x: 1, y: 1 },
            colour: Colour::White,
        }],
    };
    assert_eq!(
        solve(&puzzle),
        Some(vec![
            IntersectionOrEdge::Intersection(Pos { x: 1, y: 2 }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 1, y: 2 },
                dir: EdgeDirection::Horizontal,
            }),
            IntersectionOrEdge::Intersection(Pos { x: 2, y: 2 }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 2, y: 1 },
                dir: EdgeDirection::Vertical,
            }),
            IntersectionOrEdge::Intersection(Pos { x: 2, y: 1 }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 1, y: 1 },
                dir: EdgeDirection::Horizontal,
            }),
            IntersectionOrEdge::Intersection(Pos { x: 1, y: 1 }),
            IntersectionOrEdge::Edge(Edge {
                pos: Pos { x: 1, y: 0 },
                dir: EdgeDirection::Vertical,
            }),
            IntersectionOrEdge::Intersection(Pos { x: 1, y: 0 }),
        ],)
    );
}

#[test]
fn suns_and_squares_tricolour() {
    let puzzle = Puzzle {
        width: 3,
        height: 3,
        sources: vec![IntersectionOrEdge::Edge(Edge {
            pos: Pos { x: 1, y: 3 },
            dir: EdgeDirection::Horizontal,
        })],
        exits: vec![IntersectionOrEdge::Edge(Edge {
            pos: Pos { x: 1, y: 0 },
            dir: EdgeDirection::Horizontal,
        })],
        broken: vec![],
        dots: vec![],
        squares: vec![
            ColouredSymbol {
                pos: Pos { x: 2, y: 0 },
                colour: Colour::Pink,
            },
            ColouredSymbol {
                pos: Pos { x: 2, y: 1 },
                colour: Colour::Green,
            },
            ColouredSymbol {
                pos: Pos { x: 2, y: 2 },
                colour: Colour::White,
            },
        ],
        suns: vec![ColouredSymbol {
            pos: Pos { x: 0, y: 1 },
            colour: Colour::Pink,
        }],
    };
    assert!(matches!(solve(&puzzle), Some(_)));
}
