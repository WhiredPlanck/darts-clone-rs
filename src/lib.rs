//! Rust binding of [Darts-clone](https://github.com/s-yata/darts-clone) - a clone of Darts (Double-ARray Trie System)
//! 
//! # Installation
//！
//! ```toml
//! [dependencies]
//! darts-clone-rs = "0.1.0"
//! ```
//!
//! # Examples
//!
//! ## Build trie
//!
//! ```rust
//! use darts::DartsArrayTrie;
//! ...
//! let dic = DartsArrayTrie::new();
//! let keys: Vec<String> = ... // get keys somehow
//! let values: Vec<usize> = ... // get values somehow
//! let lengths: Vec<i32> = ... // get lengths somehow
//!
//! let result = dic.build(keys.len, &keys, None /* Some(&values) */, None /* Some(&lengths) */, None);
//! assert_eq!(Ok(()), result);
//! ...
//! ```
//!
//! ## Save and open
//!
//! ```rust
//! use darts::DartsArrayTrie;
//! ...
//! let dic = DartsArrayTrie::new();
//! // build ...
//! let dic_copy = DartsArrayTrie::new();
//! assert_eq!(Ok(), dic.save("path/to/dict", "wb", 0));
//! assert_eq!(Ok(), dic_copy.open("path/to/dict", "rb", 0, 0));
//! ...
//! ```
//!
//! ## Search
//!
//! ```rust
//! use darts::DartsArrayTrie;
//! ...
//! const MAX_RESULT_NUM: usize = 16;
//! let dic = DartsArrayTrie::new();
//! // build ...
//! let result = dic.common_prefix_search(key, MAX_RESULT_NUM, 0, 0);
//! assert_eq!(/* prefixes */, result);
//! ...
//! ```
//!
//! ## Traverse
//!
//! ```rust
//! use darts::DartsArrayTrie;
//! ...
//! let dic = DartsArrayTrie::new();
//! // build ...
//! let mut id = 0usize;
//! let mut key_pos = 0usize;
//! for i in 0..key.len() {
//!     let result = dic.traverse(key, &mut id, &mut key_pos, i + 1);
//!     assert_ne!(result, -2);
//! }
//! ...
//! ```

pub mod darts;

#[cfg(test)]
mod tests;
