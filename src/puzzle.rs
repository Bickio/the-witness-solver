#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Puzzle {
    pub width: u32,
    pub height: u32,
    pub sources: Vec<IntersectionOrEdge>,
    pub exits: Vec<IntersectionOrEdge>,
    pub broken: Vec<IntersectionOrEdge>,
    pub dots: Vec<IntersectionOrEdge>,
    pub squares: Vec<ColouredSymbol>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntersectionOrEdge {
    Intersection(Pos),
    Edge(Edge),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub pos: Pos,
    pub dir: EdgeDirection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdgeDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pos {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColouredSymbol {
    pub pos: Pos,
    pub colour: Colour,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour {
    Black,
    White,
    Pink,
    Red,
    Orange,
    Yellow,
    Green,
    Turquoise,
    Blue,
    Purple,
}
