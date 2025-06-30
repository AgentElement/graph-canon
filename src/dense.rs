use nauty_Traces_sys::{empty_graph, ADDONEARC, SETWORDSNEEDED};
use petgraph::{visit::GetAdjacencyMatrix, EdgeType, Graph};
use std::{collections::BTreeMap, ffi::c_int, hash::Hash};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct DenseGraph {
    pub g: Vec<u64>,
    pub n: usize,
    pub e: usize,
    pub m: usize,
    pub nodes: Nodes,
}
impl DenseGraph {
    pub fn from_petgraph<N, E, Ty>(graph: &Graph<N, E, Ty>) -> Self
    where
        Ty: EdgeType,
        N: Ord + Eq + Clone,
    {
        let n = graph.node_count();
        let e = graph.edge_count();
        let m = SETWORDSNEEDED(n);
        let nodes = Nodes::new(graph);
        let mut g = empty_graph(m, n);
        let adj = graph.adjacency_matrix();
        for idx in 0..n {
            for jdx in 0..n {
                if adj.contains(idx * n + jdx) {
                    ADDONEARC(&mut g, idx, jdx, m)
                }
            }
        }
        Self { g, n, e, m, nodes }
    }

    pub fn orbits(&self) -> &[i32] {
        &self.nodes.orbits
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Nodes {
    pub lab: Vec<c_int>,
    pub ptn: Vec<c_int>,
    pub orbits: Vec<c_int>,
}
impl Nodes {
    pub fn new<N, E, Ty>(graph: &Graph<N, E, Ty>) -> Self
    where
        Ty: EdgeType,
        N: Ord + Eq + Clone,
    {
        let mut buckets = BTreeMap::new();
        for (ix, weight) in graph.node_weights().enumerate() {
            let bucket = buckets.entry(weight.clone()).or_insert(vec![]);
            bucket.push(ix as i32);
        }

        let mut lab = vec![];
        let mut ptn = vec![];
        for (_, bucket) in buckets {
            ptn.extend(vec![1; bucket.len() - 1]);
            ptn.push(0);
            lab.extend(bucket);
        }
        Self {
            lab,
            ptn,
            orbits: vec![0; graph.node_count()],
        }
    }
}
