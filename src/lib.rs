#![allow(unused)]
use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasher, Hash, Hasher, BuildHasherDefault};

type DefaultBuildHasher = BuildHasherDefault<DefaultHasher>;
#[derive(PartialEq, Debug)]
struct MerkleTree<T, S> {
    hasher: S,
    head: Box<MerkleNode<T>>,
}

#[derive(PartialEq, Debug)]
enum MerkleNode<T> {
    Branch(u64),
    Leaf(T),
    Empty,
}

impl<T> MerkleTree<T, DefaultBuildHasher>
where
    T: Hash,
{
    fn new() -> Self {
        Self {
            head: Box::new(MerkleNode::Empty),
            hasher: DefaultBuildHasher::default(),
        }
    }
}

impl<T, S> MerkleTree<T,S> {
    /// Adds a new data block to the merkle tree
    fn push() {

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_same_node() {
        let tree: MerkleTree<u64, BuildHasherDefault<DefaultHasher>> = MerkleTree::new();
        assert_eq!(
            tree.head,
            Box::new(MerkleNode::Empty)
        );
        
    }
    #[test]
    fn empty_same_hash() {
        let tree: MerkleTree<u64, DefaultBuildHasher> = MerkleTree::new();
        let mut hasher_tree = tree.hasher.build_hasher();
        let mut hasher_default = DefaultBuildHasher::default().build_hasher();
        let message = b"hello";

        hasher_tree.write(message);
        hasher_default.write(message);
        assert_eq!(hasher_default.finish(), hasher_tree.finish())
    }
}
