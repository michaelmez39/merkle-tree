#![allow(unused)]
use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::hash::{BuildHasher, Hash, Hasher, BuildHasherDefault};

type DefaultBuildHasher = BuildHasherDefault<DefaultHasher>;
#[derive(PartialEq, Debug)]
struct MerkleTree<T, S> {
    hasher: S,
    head: MerkleNode<T>,
}

#[derive(PartialEq, Debug)]
struct BranchNode<T> {
    hash: u64,
    left: Box<MerkleNode<T>>,
    right: Box<MerkleNode<T>>
}

#[derive(PartialEq, Debug)]
struct LeafNode<T> {
    hash: u64,
    data: T
}

#[derive(PartialEq, Debug)]
enum MerkleNode<T> {
    Branch(BranchNode<T>),
    Leaf(LeafNode<T>),
    Empty,
}

impl<T> MerkleTree<T, DefaultBuildHasher>
where
    T: Hash,
{
    fn new() -> Self {
        Self {
            head: MerkleNode::Empty, // may want to initialize this as a branch with two empty children?
            hasher: DefaultBuildHasher::default(),
        }
    }
}

impl<T, S> MerkleTree<T,S>
where S: BuildHasher, T: Hash {
    /// Adds a new data block to the merkle tree
    fn push(&mut self, data: T) {
        let mut queue = VecDeque::new();
        queue.push_back(&mut self.head);
        while let Some(mut current) = queue.pop_front() {
            match current {
                MerkleNode::Branch(branch) => {
                    queue.push_back(&mut branch.left);
                    queue.push_back(&mut branch.right);
                }
                MerkleNode::Leaf(_)=> (),
                MerkleNode::Empty => {
                    let mut hasher = self.hasher.build_hasher();
                    data.hash(&mut hasher);
                    let mut new_node = Box::new(MerkleNode::Leaf(LeafNode {
                        data,
                        hash: hasher.finish()
                    }));
                    std::mem::swap( current, &mut new_node);
                    // TODO: Update the hashes on each of the other nodes
                    return;
                }

            }
        }
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
            MerkleNode::Empty
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
