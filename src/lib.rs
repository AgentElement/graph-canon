//! # Graph Canon
//!
//! This crate provides a graph canonization algorithm for directed and undirected graphs
//! by calling the C library [nauty](https://pallini.di.uniroma1.it/) via [nauty-Traces-sys](https://crates.io/crates/nauty-Traces-sys)
//!
//! This crate is built on top of the [petgraph](https://crates.io/crates/petgraph) crate, but is
//! considerably faster than an existing crate [nauty-pet](https://crates.io/crates/nauty-pet) that
//! uses similar techniques because it is **very** barebones.
//!
//! ## Example
//!
//! ### Hashable Labels
//! If you are just looking to create a hashable object to determine isomorphism
//! then it is simples to use the `CanonLabeling` struct.
//!
//! This can be created from a `Graph` object directly.
//!
//! #### Directed Graphs
//! ```
//! use petgraph::{Directed, Graph};
//! use graph_canon::CanonLabeling;
//!
//! let e1 = vec![(0, 1), (0, 2), (1, 2)]; // Isomorphic
//! let e2 = vec![(1, 0), (1, 2), (0, 2)]; // Isomorphic
//! let e3 = vec![(1, 0), (1, 2), (2, 1)]; // Non-Isomorphic
//!
//! let g1 = Graph::<(), (), Directed>::from_edges(&e1);
//! let g2 = Graph::<(), (), Directed>::from_edges(&e2);
//! let g3 = Graph::<(), (), Directed>::from_edges(&e3);
//!
//! let l1 = CanonLabeling::new(&g1);
//! let l2 = CanonLabeling::new(&g2);
//! let l3 = CanonLabeling::new(&g3);
//!
//! assert_eq!(l1, l2);
//! assert_ne!(l1, l3);
//! ```
//!
//! #### Undirected Graphs
//! ```
//! use petgraph::{Undirected, Graph};
//! use graph_canon::CanonLabeling;
//!
//! let e1 = vec![(0, 1), (0, 2), (1, 2)]; // Isomorphic
//! let e2 = vec![(1, 0), (1, 2), (0, 2)]; // Isomorphic
//! let e3 = vec![(1, 0), (1, 2)];         // Non-Isomorphic
//!
//! let g1 = Graph::<(), (), Undirected>::from_edges(&e1);
//! let g2 = Graph::<(), (), Undirected>::from_edges(&e2);
//! let g3 = Graph::<(), (), Undirected>::from_edges(&e3);
//!
//! let l1 = CanonLabeling::new(&g1);
//! let l2 = CanonLabeling::new(&g2);
//! let l3 = CanonLabeling::new(&g3);
//!
//! assert_eq!(l1, l2);
//! assert_ne!(l1, l3);
//! ```
//!
//! ### Recovering the automorphism group of a `Graph`
//!
//! If you're interested in the automorphism group of a graph, you can use the `autom` module.
//!
//! #### Directed Graphs
//! ```
//! use petgraph::{Directed, Graph};
//! use graph_canon::autom::AutoGroups;
//!
//! let edges = vec![(0, 1), (1, 2), (2, 0)];
//! let graph = Graph::<(), (), Directed>::from_edges(&edges);
//! let aut = AutoGroups::from_petgraph(&graph);
//!
//! assert_eq!(aut.size(), 3);
//! ```

pub mod autom;
pub mod canon;
pub mod dense;
pub use canon::CanonLabeling;
pub use dense::DenseGraph;
