#![allow(unused)]
use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::hash::{BuildHasher, Hash, Hasher, BuildHasherDefault};

type DefaultBuildHasher = BuildHasherDefault<DefaultHasher>;

#[derive(PartialEq, Debug)]
struct MerkleTree<T, S> {
    hasher: S,
    head: Option<Box<MerkleNode<T>>>
}

#[derive(PartialEq, Debug)]
struct BranchNode<T> {
    hash: u64,
    left: Box<MerkleNode<T>>,
    right: Box<MerkleNode<T>>
}

#[derive(PartialEq, Debug)]
struct LeafNode<T> {
    block_hash: u64,
    data: T
}

#[derive(PartialEq, Debug)]
enum MerkleNode<T> {
    Branch(BranchNode<T>),
    Leaf(LeafNode<T>),
    Empty
}

impl<T> Default for MerkleNode<T> {
    fn default() -> Self {
        MerkleNode::Empty
    }
}

impl<T> MerkleTree<T, DefaultBuildHasher>
where
    T: Hash,
{
    fn new() -> Self {
        Self {
            head: None, // may want to initialize this as a branch with two empty children?
            hasher: DefaultBuildHasher::default(),
        }
    }
}

struct UpdateStatus {
    updated: bool,
    hash: u64
}

impl<T, S> MerkleTree<T,S>
where S: BuildHasher, T: Hash {
    // fn push_rec(cursor: MerkleNode<T>, node: Box<MerkleNode<T>>) -> false {
    //     match cursor {
    //         MerkleNode::Branch(branch) => {
            
    //         }
    //         MerkleNode::Leaf(leaf) => {
    //             let bran
    //         }
    //     }
    // }

    fn push(&mut self, data: T) {
        let mut hasher = self.hasher.build_hasher();
        data.hash(&mut hasher);
        let mut new_node = LeafNode {
            data,
            block_hash: hasher.finish()
        };

        if let Some(mut head) = self.head {
            let mut queue = VecDeque::new();
            queue.push_back(&mut head);
            while let Some(current) = queue.pop_front() {
                match current.as_mut() {
                    MerkleNode::Branch(branch) => {
                        queue.push_back(&mut branch.left);
                        queue.push_back(&mut branch.right);
                    }
                    MerkleNode::Leaf(leaf) => {
                        let mut block_hasher = self.hasher.build_hasher();
                        leaf.block_hash.hash(&mut block_hasher);
                        new_node.block_hash.hash(&mut block_hasher);
                        let new_branch = MerkleNode::Branch(BranchNode {
                            left: std::mem::take(current),
                            right: Box::new(MerkleNode::Leaf(new_node)),
                            hash: block_hasher.finish()
                        })
                    }
                    MerkleNode::Empty => unreachable!("Tree should not have empty nodes")
                }
            }
        } else {
            self.head.replace(Box::new(MerkleNode::Leaf(new_node)));
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
            None
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
