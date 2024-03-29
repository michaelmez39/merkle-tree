#![allow(unused)]
use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};

type DefaultBuildHasher = BuildHasherDefault<DefaultHasher>;

#[derive(PartialEq, Debug)]
struct MerkleTree<T, S> {
    hasher: S,
    head: Option<Box<MerkleNode<T>>>,
}

#[derive(PartialEq, Debug)]
struct BranchNode<T> {
    hash: u64,
    left: Box<MerkleNode<T>>,
    right: Box<MerkleNode<T>>,
}

#[derive(PartialEq, Debug)]
struct LeafNode<T> {
    block_hash: u64,
    data: Box<T>,
}

#[derive(PartialEq, Debug)]
enum MerkleNode<T> {
    Branch(BranchNode<T>),
    Leaf(Option<LeafNode<T>>),
}

impl<T> MerkleNode<T> {
    fn new_leaf<S>(data: T, build_hasher: &S) -> Box<MerkleNode<T>>
    where
        T: Hash,
        S: BuildHasher,
    {
        let mut hasher = build_hasher.build_hasher();
        data.hash(&mut hasher);

        Box::new(MerkleNode::Leaf(Some(LeafNode {
            data: Box::new(data),
            block_hash: hasher.finish(),
        })))
    }

    fn get_hash(&self) -> u64 {
        match self {
            MerkleNode::Branch(branch) => branch.hash,
            MerkleNode::Leaf(Some(leaf)) => leaf.block_hash,
            _ => 0,
        }
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
    hash: u64,
}

impl<T, S> MerkleTree<T, S>
where
    S: BuildHasher,
    T: Hash,
{
    fn push(&mut self, data: T) {
        if let Some(mut head) = self.head.as_mut() {
            let mut queue = VecDeque::new();
            queue.push_back(head);
            while let Some(current) = queue.pop_front() {
                let current_borrow = current.as_mut();
                match current_borrow {
                    MerkleNode::Branch(branch) => {
                        queue.push_back(&mut branch.left);
                        queue.push_back(&mut branch.right);
                    }
                    MerkleNode::Leaf(leaf) => {
                        let mut new_leaf = MerkleNode::new_leaf(data, &self.hasher);

                        let mut hasher = self.hasher.build_hasher();
                        new_leaf.get_hash().hash(&mut hasher);
                        let mut block_hasher = self.hasher.build_hasher();
                        if let Some(LeafNode { block_hash, .. }) = leaf {
                            block_hash.hash(&mut block_hasher)
                        }

                        new_leaf.get_hash().hash(&mut block_hasher);

                        let mut new_branch = Box::new(MerkleNode::Branch(BranchNode {
                            left: Box::new(MerkleNode::Leaf(std::mem::replace(leaf, None))),
                            right: new_leaf,
                            hash: block_hasher.finish(),
                        }));

                        std::mem::swap(current_borrow, &mut new_branch);
                        return;
                    }
                }
            }
        } else {
            let mut hasher = self.hasher.build_hasher();
            data.hash(&mut hasher);
            let mut new_node = LeafNode {
                data: Box::new(data),
                block_hash: hasher.finish(),
            };
            self.head
                .replace(Box::new(MerkleNode::Leaf(Some(new_node))));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_same_node() {
        let tree: MerkleTree<u64, BuildHasherDefault<DefaultHasher>> = MerkleTree::new();
        assert_eq!(tree.head, None);
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
