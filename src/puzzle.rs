#[derive(Debug, Clone, PartialEq)]
pub struct Puzzle {
    pub width: u32,
    pub height: u32,
    pub sources: Vec<IntersectionOrEdge>,
    pub exits: Vec<IntersectionOrEdge>,
    pub broken: Vec<IntersectionOrEdge>,
    pub dots: Vec<IntersectionOrEdge>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IntersectionOrEdge {
    Intersection(Pos),
    Edge(Edge),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    pub pos: Pos,
    pub dir: EdgeDirection,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EdgeDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pos {
    pub x: u32,
    pub y: u32,
}
