use crate::puzzle::IntersectionOrEdge;
use crate::EdgeDirection;
use crate::{puzzle, Colour, Edge, Pos};
use itertools::Itertools;
use z3::ast::Ast;

#[derive(Debug, Clone)]
struct Node<'ctx> {
    broken: bool,
    source: bool,
    exit: bool,
    dot: bool,
    has_line: z3::ast::Bool<'ctx>,
    line_index: z3::ast::Int<'ctx>,
    source_used: z3::ast::Bool<'ctx>,
    exit_used: z3::ast::Bool<'ctx>,
}

impl<'ctx> Node<'ctx> {
    fn default(ctx: &'ctx z3::Context) -> Self {
        Node {
            broken: false,
            source: false,
            exit: false,
            dot: false,
            has_line: z3::ast::Bool::fresh_const(ctx, "has_line"),
            line_index: z3::ast::Int::fresh_const(ctx, "line_index"),
            source_used: z3::ast::Bool::fresh_const(ctx, "source_used"),
            exit_used: z3::ast::Bool::fresh_const(ctx, "exit_used"),
        }
    }
}

#[derive(Debug, Clone)]
enum Symbol {
    Square(Colour),
}

#[derive(Debug, Clone)]
struct Cell<'ctx> {
    symbol: Option<Symbol>,
    region: z3::ast::Int<'ctx>,
}

impl<'ctx> Cell<'ctx> {
    fn default(ctx: &'ctx z3::Context) -> Self {
        Cell {
            symbol: None,
            region: z3::ast::Int::fresh_const(ctx, "exit_used"),
        }
    }
}

#[derive(Debug)]
struct PuzzleModel<'ctx> {
    ctx: &'ctx z3::Context,
    width: u32,
    height: u32,
    intersections: Vec<Vec<Node<'ctx>>>,
    horizontal_edges: Vec<Vec<Node<'ctx>>>,
    vertical_edges: Vec<Vec<Node<'ctx>>>,
    cells: Vec<Vec<Cell<'ctx>>>,
    num_regions: z3::ast::Int<'ctx>,
    region_square_colours: z3::ast::Array<'ctx>,
}

impl<'ctx> PuzzleModel<'ctx> {
    fn intersection(&self, pos: &Pos) -> &Node<'ctx> {
        &self.intersections[pos.y as usize][pos.x as usize]
    }

    fn intersection_mut(&mut self, pos: &Pos) -> &mut Node<'ctx> {
        &mut self.intersections[pos.y as usize][pos.x as usize]
    }

    fn edge(&self, edge: &Edge) -> &Node<'ctx> {
        match edge.dir {
            EdgeDirection::Horizontal => {
                &self.horizontal_edges[edge.pos.y as usize][edge.pos.x as usize]
            }
            EdgeDirection::Vertical => {
                &self.vertical_edges[edge.pos.y as usize][edge.pos.x as usize]
            }
        }
    }

    fn edge_mut(&mut self, edge: &Edge) -> &mut Node<'ctx> {
        match edge.dir {
            EdgeDirection::Horizontal => {
                &mut self.horizontal_edges[edge.pos.y as usize][edge.pos.x as usize]
            }
            EdgeDirection::Vertical => {
                &mut self.vertical_edges[edge.pos.y as usize][edge.pos.x as usize]
            }
        }
    }

    fn node(&self, intersection_or_edge: &IntersectionOrEdge) -> &Node<'ctx> {
        match intersection_or_edge {
            IntersectionOrEdge::Intersection(intersection) => self.intersection(intersection),
            IntersectionOrEdge::Edge(edge) => self.edge(edge),
        }
    }

    fn node_mut(&mut self, intersection_or_edge: &IntersectionOrEdge) -> &mut Node<'ctx> {
        match intersection_or_edge {
            IntersectionOrEdge::Intersection(intersection) => self.intersection_mut(intersection),
            IntersectionOrEdge::Edge(edge) => self.edge_mut(edge),
        }
    }

    fn cell(&self, pos: &Pos) -> &Cell<'ctx> {
        &self.cells[pos.y as usize][pos.x as usize]
    }

    fn cell_mut(&mut self, pos: &Pos) -> &mut Cell<'ctx> {
        &mut self.cells[pos.y as usize][pos.x as usize]
    }

    fn adjacent(&self, intersection_or_edge: &IntersectionOrEdge) -> Vec<IntersectionOrEdge> {
        match intersection_or_edge {
            IntersectionOrEdge::Intersection(intersection) => self
                .adjacent_edges(intersection)
                .into_iter()
                .map(IntersectionOrEdge::Edge)
                .collect(),
            IntersectionOrEdge::Edge(edge) => self
                .adjacent_intersections(edge)
                .into_iter()
                .map(IntersectionOrEdge::Intersection)
                .collect(),
        }
    }

    fn adjacent_nodes(&self, intersection_or_edge: &IntersectionOrEdge) -> Vec<&Node> {
        return self
            .adjacent(intersection_or_edge)
            .iter()
            .map(|adj| self.node(adj))
            .collect();
    }

    fn adjacent_cells(&self, edge: &Edge) -> Vec<Pos> {
        let mut cells = Vec::new();
        match edge.dir {
            // 00112
            // #-#-#
            EdgeDirection::Vertical => {
                if edge.pos.x > 0 {
                    cells.push(Pos {
                        x: edge.pos.x - 1,
                        y: edge.pos.y,
                    });
                }
                if edge.pos.x < self.width {
                    cells.push(Pos {
                        x: edge.pos.x,
                        y: edge.pos.y,
                    });
                }
            }
            EdgeDirection::Horizontal => {
                if edge.pos.y > 0 {
                    cells.push(Pos {
                        x: edge.pos.x,
                        y: edge.pos.y - 1,
                    });
                }
                if edge.pos.y < self.height {
                    cells.push(Pos {
                        x: edge.pos.x,
                        y: edge.pos.y,
                    });
                }
            }
        }
        cells
    }

    fn adjacent_edges(&self, pos: &Pos) -> Vec<Edge> {
        let mut adjacent_edges: Vec<Edge> = Vec::new();
        // Left edge
        if pos.x > 0 {
            adjacent_edges.push(Edge {
                pos: Pos {
                    x: pos.x - 1,
                    y: pos.y,
                },
                dir: EdgeDirection::Horizontal,
            });
        }
        // Top edge
        if pos.y > 0 {
            adjacent_edges.push(Edge {
                pos: Pos {
                    x: pos.x,
                    y: pos.y - 1,
                },
                dir: EdgeDirection::Vertical,
            });
        }
        // Right edge
        if pos.x < self.width {
            adjacent_edges.push(Edge {
                pos: Pos { x: pos.x, y: pos.y },
                dir: EdgeDirection::Horizontal,
            });
        }
        // Bottom edge
        if pos.y < self.height {
            adjacent_edges.push(Edge {
                pos: Pos { x: pos.x, y: pos.y },
                dir: EdgeDirection::Vertical,
            });
        }
        adjacent_edges
    }

    fn adjacent_intersections(&self, edge: &Edge) -> Vec<Pos> {
        let start = edge.pos.clone();
        let end = match edge.dir {
            EdgeDirection::Vertical => Pos {
                x: edge.pos.x,
                y: edge.pos.y + 1,
            },
            EdgeDirection::Horizontal => Pos {
                x: edge.pos.x + 1,
                y: edge.pos.y,
            },
        };
        vec![start, end]
    }

    fn from_puzzle(p: &puzzle::Puzzle, ctx: &'ctx z3::Context) -> Self {
        let mut model = PuzzleModel {
            ctx,
            width: p.width,
            height: p.height,
            intersections: Self::create_2d_vec(p.width + 1, p.height + 1, || Node::default(ctx)),
            horizontal_edges: Self::create_2d_vec(p.width, p.height + 1, || Node::default(ctx)),
            vertical_edges: Self::create_2d_vec(p.width + 1, p.height, || Node::default(ctx)),
            cells: Self::create_2d_vec(p.width, p.height, || Cell::default(ctx)),
            num_regions: z3::ast::Int::fresh_const(ctx, "num_regions"),
            region_square_colours: z3::ast::Array::fresh_const(
                ctx,
                "region_square_colours",
                &z3::Sort::int(ctx),
                &z3::Sort::int(ctx),
            ),
        };
        model.add_broken(&p.broken);
        model.add_sources(&p.sources);
        model.add_exits(&p.exits);
        model.add_dots(&p.dots);
        model.add_squares(&p.squares);
        model
    }

    fn create_2d_vec<T, F: Fn() -> T>(width: u32, height: u32, constructor: F) -> Vec<Vec<T>> {
        (0..(height + 1))
            .map(|_| (0..(width + 1)).map(|_| constructor()).collect())
            .collect()
    }

    fn add_broken(&mut self, broken: &Vec<IntersectionOrEdge>) {
        for b in broken {
            self.node_mut(b).broken = true;
        }
    }

    fn add_sources(&mut self, sources: &Vec<IntersectionOrEdge>) {
        for s in sources {
            self.node_mut(s).source = true;
        }
    }

    fn add_exits(&mut self, exits: &Vec<IntersectionOrEdge>) {
        for e in exits {
            self.node_mut(e).exit = true;
        }
    }

    fn add_dots(&mut self, dots: &Vec<IntersectionOrEdge>) {
        for d in dots {
            self.node_mut(d).dot = true;
        }
    }

    fn add_squares(&mut self, squares: &[puzzle::ColouredSymbol]) {
        for s in squares {
            self.cell_mut(&s.pos).symbol = Some(Symbol::Square(s.colour));
        }
    }

    fn intersections_and_edges(&self) -> Vec<IntersectionOrEdge> {
        let mut intersections: Vec<_> = self
            .intersections()
            .into_iter()
            .map(IntersectionOrEdge::Intersection)
            .collect();
        let mut edges = self
            .edges()
            .into_iter()
            .map(IntersectionOrEdge::Edge)
            .collect();
        intersections.append(&mut edges);
        intersections
    }

    fn intersections(&self) -> Vec<Pos> {
        let mut result: Vec<Pos> = Vec::new();
        for x in 0..(self.width + 1) {
            for y in 0..(self.height + 1) {
                result.push(Pos { x, y });
            }
        }
        result
    }

    fn edges(&self) -> Vec<Edge> {
        let mut result: Vec<Edge> = Vec::new();
        for x in 0..(self.width + 1) {
            for y in 0..(self.height + 1) {
                if x < self.width {
                    result.push(Edge {
                        pos: Pos { x, y },
                        dir: EdgeDirection::Horizontal,
                    });
                }
                if y < self.height {
                    result.push(Edge {
                        pos: Pos { x, y },
                        dir: EdgeDirection::Vertical,
                    });
                }
            }
        }
        result
    }

    fn cell_positions(&self) -> Vec<Pos> {
        let mut result: Vec<Pos> = Vec::new();
        for x in 0..(self.width) {
            for y in 0..(self.height) {
                result.push(Pos { x, y })
            }
        }
        result
    }

    fn edges_from_border(&self) -> Vec<Edge> {
        let mut result: Vec<Edge> = Vec::new();
        for x in 1..(self.width) {
            result.push(Edge {
                dir: EdgeDirection::Vertical,
                pos: Pos { x, y: 0 },
            });
            result.push(Edge {
                dir: EdgeDirection::Vertical,
                pos: Pos {
                    x,
                    y: self.height - 1,
                },
            });
        }
        for y in 1..(self.height) {
            result.push(Edge {
                dir: EdgeDirection::Horizontal,
                pos: Pos { x: 0, y },
            });
            result.push(Edge {
                dir: EdgeDirection::Horizontal,
                pos: Pos {
                    x: self.width - 1,
                    y,
                },
            });
        }
        result
    }

    fn is_external(&self, intersection_or_edge: &IntersectionOrEdge) -> bool {
        match intersection_or_edge {
            IntersectionOrEdge::Intersection(pos) => {
                pos.x == 0 || pos.x == self.width || pos.y == 0 || pos.y == self.height
            }
            IntersectionOrEdge::Edge(edge) => match edge.dir {
                EdgeDirection::Vertical => edge.pos.x == 0 || edge.pos.x == self.width,
                EdgeDirection::Horizontal => edge.pos.y == 0 || edge.pos.y == self.height,
            },
        }
    }

    fn constrain(&self, solver: &z3::Solver) {
        let mut sources: Vec<IntersectionOrEdge> = Vec::new();
        let mut exits: Vec<IntersectionOrEdge> = Vec::new();
        for intersection_or_edge in self.intersections_and_edges() {
            let node = self.node(&intersection_or_edge);
            if node.source {
                sources.push(intersection_or_edge.clone());
            }
            if node.exit {
                exits.push(intersection_or_edge.clone());
            }
            self.constrain_intersection_or_edge(solver, &intersection_or_edge);
        }
        self.constrain_sources_and_exits(solver, &sources, &exits);
        self.constrain_regions(solver, &sources, &exits);
        self.constrain_symbols(solver);
    }

    fn constrain_intersection_or_edge(
        &self,
        solver: &z3::Solver,
        intersection_or_edge: &IntersectionOrEdge,
    ) {
        let node = self.node(intersection_or_edge);
        if node.broken {
            solver.assert(&!&node.has_line);
        }
        if !node.source {
            solver.assert(&!&node.source_used);
        }
        if !node.exit {
            solver.assert(&!&node.exit_used);
        }
        if node.dot {
            solver.assert(&node.has_line);
        }

        let adjacent_nodes = self.adjacent_nodes(intersection_or_edge);
        let adjacent_nodes_with_line = adjacent_nodes
            .iter()
            .map(|adj| (&adj.has_line, 1))
            .collect::<Vec<_>>();
        let zero_adjacent_lines = z3::ast::Bool::pb_eq(self.ctx, &adjacent_nodes_with_line, 0);
        let one_adjacent_line = z3::ast::Bool::pb_eq(self.ctx, &adjacent_nodes_with_line, 1);
        let two_adjacent_lines = z3::ast::Bool::pb_eq(self.ctx, &adjacent_nodes_with_line, 2);

        let one = z3::ast::Int::from_i64(self.ctx, 1);
        let consecutive_numbers = adjacent_nodes
            .into_iter()
            .combinations(2)
            .map(|pair| {
                let pair_has_line = &pair[0].has_line & &pair[1].has_line;
                let increasing = &node.line_index._eq(&(&pair[0].line_index - &one))
                    & &node.line_index._eq(&(&pair[1].line_index + &one));
                let decreasing = &node.line_index._eq(&(&pair[0].line_index + &one))
                    & &node.line_index._eq(&(&pair[1].line_index - &one));
                pair_has_line.implies(&(&increasing | &decreasing))
            })
            .reduce(|acc, condition| acc & condition)
            .unwrap_or_else(|| z3::ast::Bool::from_bool(self.ctx, false));

        let is_source_or_exit = &node.exit_used | &node.source_used;
        let is_source_and_exit = &node.exit_used & &node.source_used;

        let valid_not_in_line = !&node.has_line & !&is_source_or_exit;
        let valid_middle_of_line =
            &node.has_line & !&is_source_or_exit & &two_adjacent_lines & &consecutive_numbers;
        let valid_end_of_line = &node.has_line & &is_source_or_exit & &one_adjacent_line;
        let valid_entire_line = &node.has_line & &is_source_and_exit & &zero_adjacent_lines;
        solver.assert(
            &(valid_not_in_line | valid_middle_of_line | valid_end_of_line | valid_entire_line),
        );
    }

    fn constrain_sources_and_exits(
        &self,
        solver: &z3::Solver,
        sources: &[IntersectionOrEdge],
        exits: &[IntersectionOrEdge],
    ) {
        solver.assert(&z3::ast::Bool::pb_eq(
            self.ctx,
            &sources
                .iter()
                .map(|source| (&self.node(source).source_used, 1))
                .collect::<Vec<_>>(),
            1,
        ));
        solver.assert(&z3::ast::Bool::pb_eq(
            self.ctx,
            &exits
                .iter()
                .map(|exit| (&self.node(exit).exit_used, 1))
                .collect::<Vec<_>>(),
            1,
        ));
    }

    fn constrain_regions(
        &self,
        solver: &z3::Solver,
        sources: &[IntersectionOrEdge],
        exits: &[IntersectionOrEdge],
    ) {
        let zero = z3::ast::Int::from_u64(self.ctx, 0);
        let one = z3::ast::Int::from_u64(self.ctx, 1);

        for cell_pos in self.cell_positions() {
            let cell = self.cell(&cell_pos);
            let cell_region = &cell.region;
            solver.assert(&(cell_region.ge(&zero)));
            solver.assert(&(cell_region.lt(&self.num_regions)));
        }
        for edge in self.edges() {
            let adj_cells = self.adjacent_cells(&edge);
            if adj_cells.len() == 2 {
                let region_a = &self.cell(&adj_cells[0]).region;
                let region_b = &self.cell(&adj_cells[1]).region;
                solver.assert(
                    &self
                        .edge(&edge)
                        .has_line
                        .not()
                        .implies(&region_a._eq(region_b)),
                );
            }
        }
        let region = z3::ast::Int::fresh_const(self.ctx, "region");
        let valid_region = &region.ge(&zero) & &region.lt(&self.num_regions);
        let cells_in_region: Vec<_> = self
            .cell_positions()
            .iter()
            .map(|pos| self.cell(pos).region._eq(&region))
            .collect();
        solver.assert(&z3::ast::forall_const(
            self.ctx,
            &[&region],
            &[],
            &valid_region.implies(&z3::ast::Bool::pb_ge(
                self.ctx,
                cells_in_region
                    .iter()
                    .map(|cond| (cond, 1))
                    .collect::<Vec<_>>()
                    .as_ref(),
                1,
            )),
        ));

        let internal_terminations = sources
            .iter()
            .chain(exits.iter())
            .filter(|intersection_or_edge| !self.is_external(intersection_or_edge))
            .map(|intersection_or_edge| self.node(intersection_or_edge))
            .map(|node| (&node.source_used ^ &node.exit_used).ite(&one, &zero))
            .reduce(|accum, num| accum + num)
            .unwrap_or_else(|| z3::ast::Int::from_i64(self.ctx, 0));
        let edge_lines_from_border = self
            .edges_from_border()
            .iter()
            .map(|edge| self.edge(edge).has_line.ite(&one, &zero))
            .reduce(|accum, num| accum + num)
            .unwrap_or_else(|| z3::ast::Int::from_i64(self.ctx, 0));
        let mut num_regions = (edge_lines_from_border - internal_terminations)
            .div(&z3::ast::Int::from_i64(self.ctx, 2))
            + &one;
        num_regions = num_regions.le(&zero).ite(&one, &num_regions);
        if self.width > 0 && self.height > 0 {
            solver.assert(&self.num_regions._eq(&num_regions));
        };
    }

    fn constrain_symbols(&self, solver: &z3::Solver) {
        for pos in self.cell_positions() {
            let cell = self.cell(&pos);
            match &cell.symbol {
                Some(symbol) => match symbol {
                    Symbol::Square(colour) => {
                        let region_square_color = self
                            .region_square_colours
                            .select(&cell.region)
                            .as_int()
                            .unwrap();
                        solver.assert(
                            &region_square_color
                                ._eq(&z3::ast::Int::from_i64(self.ctx, *colour as i64)),
                        );
                    }
                },
                None => {}
            }
        }
    }

    fn extract_line(&self, model: &z3::Model) -> Vec<IntersectionOrEdge> {
        let mut line = Vec::new();
        let mut current = self.extract_line_start(model);
        line.push(current.clone());
        while !model
            .eval(&self.node(&current).exit_used, true)
            .unwrap()
            .as_bool()
            .unwrap()
        {
            current = self
                .adjacent(&current)
                .into_iter()
                .find(|adj| {
                    if line.contains(adj) {
                        return false;
                    }
                    if model
                        .eval(&self.node(adj).has_line, true)
                        .unwrap()
                        .as_bool()
                        .unwrap()
                    {
                        return true;
                    }
                    false
                })
                .expect("No continuation of line found");
            line.push(current.clone());
        }
        line
    }

    fn extract_line_start(&self, model: &z3::Model) -> IntersectionOrEdge {
        self.intersections_and_edges()
            .into_iter()
            .find(|intersection_or_edge| {
                model
                    .eval(&self.node(intersection_or_edge).source_used, true)
                    .unwrap()
                    .as_bool()
                    .unwrap()
            })
            .expect("Could not find start of line")
    }
}

pub fn solve(puzzle: &puzzle::Puzzle) -> Option<Vec<IntersectionOrEdge>> {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let puzzle_model = PuzzleModel::from_puzzle(puzzle, &ctx);
    let solver = z3::Solver::new(&ctx);
    puzzle_model.constrain(&solver);
    match solver.check() {
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            Some(puzzle_model.extract_line(&model))
        }
        _ => None,
    }
}
