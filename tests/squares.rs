use witness::{
    solve, Colour, ColouredSymbol, Edge, EdgeDirection, IntersectionOrEdge, Pos, Puzzle,
};

#[test]
fn two_squares_sat() {
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
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 0, y: 1 },
                colour: Colour::White,
            },
        ],
    };
    assert!(matches!(solve(&puzzle), Some(_)));
}

#[test]
fn two_squares_unsat() {
    let puzzle = Puzzle {
        width: 1,
        height: 2,
        sources: vec![IntersectionOrEdge::Intersection(Pos { x: 0, y: 1 })],
        exits: vec![IntersectionOrEdge::Intersection(Pos { x: 1, y: 1 })],
        broken: vec![IntersectionOrEdge::Edge(Edge {
            pos: Pos { x: 0, y: 1 },
            dir: EdgeDirection::Horizontal,
        })],
        dots: vec![],
        squares: vec![
            ColouredSymbol {
                pos: Pos { x: 0, y: 0 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 0, y: 1 },
                colour: Colour::White,
            },
        ],
    };
    assert_eq!(solve(&puzzle), None);
}

#[test]
fn quarry_door_sat() {
    let puzzle = Puzzle {
        width: 4,
        height: 4,
        sources: vec![IntersectionOrEdge::Intersection(Pos { x: 0, y: 4 })],
        exits: vec![IntersectionOrEdge::Intersection(Pos { x: 4, y: 0 })],
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
            ColouredSymbol {
                pos: Pos { x: 2, y: 0 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 3, y: 0 },
                colour: Colour::White,
            },
            ColouredSymbol {
                pos: Pos { x: 0, y: 1 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 1, y: 1 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 2, y: 1 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 3, y: 1 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 0, y: 2 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 1, y: 2 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 2, y: 2 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 3, y: 2 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 0, y: 3 },
                colour: Colour::White,
            },
            ColouredSymbol {
                pos: Pos { x: 1, y: 3 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 2, y: 3 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 3, y: 3 },
                colour: Colour::White,
            },
        ],
    };
    assert!(matches!(solve(&puzzle), Some(_)));
}

#[test]
fn quarry_door_unsat() {
    let puzzle = Puzzle {
        width: 4,
        height: 4,
        sources: vec![IntersectionOrEdge::Intersection(Pos { x: 0, y: 4 })],
        exits: vec![IntersectionOrEdge::Intersection(Pos { x: 4, y: 0 })],
        broken: vec![IntersectionOrEdge::Intersection(Pos { x: 2, y: 2 })],
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
            ColouredSymbol {
                pos: Pos { x: 2, y: 0 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 3, y: 0 },
                colour: Colour::White,
            },
            ColouredSymbol {
                pos: Pos { x: 0, y: 1 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 1, y: 1 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 2, y: 1 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 3, y: 1 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 0, y: 2 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 1, y: 2 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 2, y: 2 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 3, y: 2 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 0, y: 3 },
                colour: Colour::White,
            },
            ColouredSymbol {
                pos: Pos { x: 1, y: 3 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 2, y: 3 },
                colour: Colour::Black,
            },
            ColouredSymbol {
                pos: Pos { x: 3, y: 3 },
                colour: Colour::White,
            },
        ],
    };
    let result = solve(&puzzle);
    println!("{:#?}", result);
    assert!(matches!(result, None));
}
