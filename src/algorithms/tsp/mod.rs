//! A solver for the Travelling Salesman Problem.
//!
//! Based on code developed at ETH by Christoph von Praun, Florian
//! Schneider, Nicholas Matsakis, and Thomas Gross.

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod graph;
mod parser;
mod solver;
mod step;
mod tour;
mod weight;

use self::graph::{Graph, Node};
use self::solver::SolverCx;
use crate::Benchable;

pub struct Tsp {
    graph: Graph,
}

const SEQ_THRESHOLD: usize = 8;
const FROM: usize = 0;

impl Tsp {
    pub fn new() -> Self {
        let path = Path::new("data/tsp/dj10.tsp");
        let graph = parse_graph(path).unwrap();

        assert!(FROM < graph.num_nodes());

        Self { graph }
    }
}

impl Benchable for Tsp {
    fn name(&self) -> &'static str {
        "Travelling Salesman Problem"
    }

    fn execute(&mut self) {
        let mut solver = SolverCx::new(&self.graph, SEQ_THRESHOLD);
        solver.search_from(Node::new(FROM));
    }
}

fn parse_graph(datafile: &Path) -> Result<Graph, Box<dyn Error>> {
    let mut file = File::open(datafile)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    let graph = parser::parse_tsp_data(&text)?;
    Ok(graph)
}
