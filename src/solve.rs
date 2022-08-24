use crate::puzzle;
use crate::puzzle::EdgeDirection;
use crate::puzzle::IntersectionOrEdge;

#[derive(Debug, Clone)]
struct Node<'ctx> {
    broken: bool,
    source: bool,
    exit: bool,
    dot: bool,
    has_line: z3::ast::Bool<'ctx>,
    source_used: z3::ast::Bool<'ctx>,
    exit_used: z3::ast::Bool<'ctx>,
}

#[derive(Debug)]
struct PuzzleModel<'ctx> {
    ctx: &'ctx z3::Context,
    width: u32,
    height: u32,
    intersections: Vec<Vec<Node<'ctx>>>,
    horizontal_edges: Vec<Vec<Node<'ctx>>>,
    vertical_edges: Vec<Vec<Node<'ctx>>>,
}

impl<'ctx> PuzzleModel<'ctx> {
    fn intersection(&self, pos: &puzzle::Pos) -> &Node<'ctx> {
        &self.intersections[pos.y as usize][pos.x as usize]
    }

    fn intersection_mut(&mut self, pos: &puzzle::Pos) -> &mut Node<'ctx> {
        &mut self.intersections[pos.y as usize][pos.x as usize]
    }

    fn edge(&self, edge: &puzzle::Edge) -> &Node<'ctx> {
        match edge.dir {
            EdgeDirection::Horizontal => {
                &self.horizontal_edges[edge.pos.y as usize][edge.pos.x as usize]
            }
            EdgeDirection::Vertical => {
                &self.vertical_edges[edge.pos.y as usize][edge.pos.x as usize]
            }
        }
    }

    fn edge_mut(&mut self, edge: &puzzle::Edge) -> &mut Node<'ctx> {
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

    fn adjacent(&self, intersection_or_edge: &IntersectionOrEdge) -> Vec<IntersectionOrEdge> {
        match intersection_or_edge {
            IntersectionOrEdge::Intersection(intersection) => self
                .adjacent_edges(intersection)
                .into_iter()
                .map(|edge| IntersectionOrEdge::Edge(edge))
                .collect(),
            IntersectionOrEdge::Edge(edge) => self
                .adjacent_intersections(edge)
                .into_iter()
                .map(|intersection| IntersectionOrEdge::Intersection(intersection))
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

    fn adjacent_edges(&self, pos: &puzzle::Pos) -> Vec<puzzle::Edge> {
        let mut adjacent_edges: Vec<puzzle::Edge> = Vec::new();
        // Left edge
        if pos.x > 0 {
            adjacent_edges.push(puzzle::Edge {
                pos: puzzle::Pos {
                    x: pos.x - 1,
                    y: pos.y,
                },
                dir: EdgeDirection::Horizontal,
            });
        }
        // Top edge
        if pos.y > 0 {
            adjacent_edges.push(puzzle::Edge {
                pos: puzzle::Pos {
                    x: pos.x,
                    y: pos.y - 1,
                },
                dir: EdgeDirection::Vertical,
            });
        }
        // Right edge
        if pos.x < self.width {
            adjacent_edges.push(puzzle::Edge {
                pos: puzzle::Pos { x: pos.x, y: pos.y },
                dir: EdgeDirection::Horizontal,
            });
        }
        // Bottom edge
        if pos.y < self.height {
            adjacent_edges.push(puzzle::Edge {
                pos: puzzle::Pos { x: pos.x, y: pos.y },
                dir: EdgeDirection::Vertical,
            });
        }
        adjacent_edges
    }

    fn adjacent_intersections(&self, edge: &puzzle::Edge) -> Vec<puzzle::Pos> {
        let start = edge.pos.clone();
        let end = match edge.dir {
            EdgeDirection::Vertical => puzzle::Pos {
                x: edge.pos.x,
                y: edge.pos.y + 1,
            },
            EdgeDirection::Horizontal => puzzle::Pos {
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
            intersections: Self::create_node_2d_vec(&ctx, p.width + 1, p.height + 1),
            horizontal_edges: Self::create_node_2d_vec(&ctx, p.width, p.height + 1),
            vertical_edges: Self::create_node_2d_vec(&ctx, p.width + 1, p.height),
        };
        model.add_broken(&p.broken);
        model.add_sources(&p.sources);
        model.add_exits(&p.exits);
        model.add_dots(&p.dots);
        model
    }

    fn create_node_2d_vec(ctx: &'ctx z3::Context, width: u32, height: u32) -> Vec<Vec<Node<'ctx>>> {
        (0..(height + 1))
            .map(|_| {
                (0..(width + 1))
                    .map(|_| Node {
                        broken: false,
                        source: false,
                        exit: false,
                        dot: false,
                        has_line: z3::ast::Bool::fresh_const(&ctx, "has_line"),
                        source_used: z3::ast::Bool::fresh_const(&ctx, "source_used"),
                        exit_used: z3::ast::Bool::fresh_const(&ctx, "exit_used"),
                    })
                    .collect()
            })
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

    fn intersections_and_edges(&self) -> Vec<IntersectionOrEdge> {
        let mut result: Vec<IntersectionOrEdge> = Vec::new();
        for x in 0..(self.width + 1) {
            for y in 0..(self.height + 1) {
                result.push(IntersectionOrEdge::Intersection(puzzle::Pos { x, y }));
                if x < self.width {
                    result.push(IntersectionOrEdge::Edge(puzzle::Edge {
                        pos: puzzle::Pos { x, y },
                        dir: EdgeDirection::Horizontal,
                    }));
                }
                if y < self.height {
                    result.push(IntersectionOrEdge::Edge(puzzle::Edge {
                        pos: puzzle::Pos { x, y },
                        dir: EdgeDirection::Vertical,
                    }));
                }
            }
        }
        result
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
        self.constrain_sources_and_exits(solver, sources, exits);
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
        let zero_adjacent_lines = z3::ast::Bool::pb_eq(&self.ctx, &adjacent_nodes_with_line, 0);
        let one_adjacent_line = z3::ast::Bool::pb_eq(&self.ctx, &adjacent_nodes_with_line, 1);
        let two_adjacent_lines = z3::ast::Bool::pb_eq(&self.ctx, &adjacent_nodes_with_line, 2);

        let is_source_or_exit = &node.exit_used | &node.source_used;
        let is_source_and_exit = &node.exit_used & &node.source_used;

        let valid_not_in_line = !&node.has_line & !&is_source_or_exit;
        let valid_middle_of_line = &node.has_line & !&is_source_or_exit & &two_adjacent_lines;
        let valid_end_of_line = &node.has_line & &is_source_or_exit & &one_adjacent_line;
        let valid_entire_line = &node.has_line & &is_source_and_exit & &zero_adjacent_lines;
        solver.assert(
            &(valid_not_in_line | valid_middle_of_line | valid_end_of_line | valid_entire_line),
        );
    }

    fn constrain_sources_and_exits(
        &self,
        solver: &z3::Solver,
        sources: Vec<IntersectionOrEdge>,
        exits: Vec<IntersectionOrEdge>,
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
                        .eval(&self.node(&adj).has_line, true)
                        .unwrap()
                        .as_bool()
                        .unwrap()
                    {
                        return true;
                    }
                    return false;
                })
                .expect("No continuation of line found");
            line.push(current.clone());
        }
        line
    }

    fn extract_line_start(&self, model: &z3::Model) -> IntersectionOrEdge {
        return self
            .intersections_and_edges()
            .into_iter()
            .find(|intersection_or_edge| {
                model
                    .eval(&self.node(intersection_or_edge).source_used, true)
                    .unwrap()
                    .as_bool()
                    .unwrap()
            })
            .expect("Could not find start of line");
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
