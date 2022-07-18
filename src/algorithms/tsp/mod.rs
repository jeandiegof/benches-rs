//! A solver for the Travelling Salesman Problem.
//!
//! Based on code developed at ETH by Christoph von Praun, Florian
//! Schneider, Nicholas Matsakis, and Thomas Gross.

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

mod graph;
mod parser;
mod solver;
mod step;
mod tour;
mod weight;

use self::graph::{Graph, Node};
use self::solver::SolverCx;
use {crate::BenchableExt, pinscher::Benchable, std::thread};

pub struct Tsp {
    graph: Graph,
    run: usize,
}

const SEQ_THRESHOLD: usize = 8;
const FROM: usize = 0;

impl Tsp {
    const THREADS_TO_MAXIMUM_SPEEDUP: usize = 32;

    pub fn new() -> Self {
        let path = Path::new("data/tsp/dj10.tsp");
        let graph = parse_graph(path).unwrap();

        assert!(FROM < graph.num_nodes());

        Self { graph, run: 0 }
    }
}

impl Benchable for Tsp {
    fn name(&self) -> &'static str {
        "Travelling Salesman Problem"
    }

    fn execute(&mut self) {
        let filename = &format!("tsp-{}.json", self.run);
        diam::gantt_json(filename, || {
            let begin = Instant::now();
            let mut solver = SolverCx::new(&self.graph, SEQ_THRESHOLD);
            solver.search_from(Node::new(FROM));
            println!("{}: {}", self.run, begin.elapsed().as_micros())
        })
        .unwrap();

        self.run = self.run + 1;
    }
}

impl BenchableExt for Tsp {
    fn execution_threads(&self) -> usize {
        let available_parallelism = thread::available_parallelism().unwrap();

        Self::THREADS_TO_MAXIMUM_SPEEDUP.min(usize::from(available_parallelism))
    }
}

fn parse_graph(datafile: &Path) -> Result<Graph, Box<dyn Error>> {
    let mut file = File::open(datafile)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    let graph = parser::parse_tsp_data(&text)?;
    Ok(graph)
}
