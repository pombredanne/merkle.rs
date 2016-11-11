
//! *merkle* implements a Merkle Tree in Rust.

#[doc(no_inline)]
pub extern crate crypto;

mod tree;
pub use tree::{ Tree };
mod hashable;
pub use hashable::{ Hashable };
mod merkletree;
pub use merkletree::{ MerkleTree };
mod merkledigest;
pub use merkledigest::{ MerkleDigest };

#[cfg(test)]
mod tests;
