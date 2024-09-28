//! [Darts-clone](https://github.com/s-yata/darts-clone) is a clone of Darts (Double-ARray Trie System) which is a C++ header library for a static double-array trie structure.
//! And here is the Rust binding for it.
//!
//! The features of Darts-clone are as follows:
//!
//! * Half-size elements
//!   * Darts-clone uses 32-bit elements and Darts uses 64-bit elements. This feature simply halves the size of dictionaries.
//! * Directed Acyclic Word Graph (DAWG)
//!   * Darts uses a basic trie to implement a dictionary. On the other hand, Darts-clone uses a Directed Acyclic Word Graph (DAWG) which is derived from a basic trie by merging its common subtrees. Darts-clone thus requires less elements than Darts if a given keyset contains many duplicate values.
//!
//! Due to these features, Darts-clone can achieve better space efficiency without degrading the search performance.

pub mod darts;

#[cfg(test)]
mod tests;
