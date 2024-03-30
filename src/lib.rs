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

// Maybe should store the data in a Vec and build the tree from the vec
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

impl<T, S> MerkleTree<T, S>
where
    S: BuildHasher,
    T: Hash,
{
    fn debug(&self)
    where
        T: std::fmt::Debug,
    {
        match &self.head {
            Some(node) => Self::debug_rec(&node),
            None => println!("Empty tree"),
        }
    }

    fn debug_rec(node: &MerkleNode<T>)
    where
        T: std::fmt::Debug,
    {
        let mut queue = VecDeque::new();
        queue.push_back((node, 0));
        while let Some((node, level)) = queue.pop_front() {
            match node {
                MerkleNode::Branch(branch) => {
                    println!("{0:0$} Branch: {1:?}", level, branch.hash);
                    queue.push_front((&branch.right, level + 1));
                    queue.push_front((&branch.left, level + 1));
                }
                MerkleNode::Leaf(Some(leaf)) => {
                    println!(
                        "{0:0$} Leaf: {1:?} {2:?} ",
                        level, leaf.block_hash, leaf.data
                    )
                }
                MerkleNode::Leaf(None) => println!("{0:0$} Empty Leaf", level),
            }
        }
    }

    fn push(&mut self, data: T) {
        if let Some(mut head) = self.head.as_mut() {
            let mut queue = VecDeque::from([head]);
            while let Some(current) = queue.pop_front() {
                let current_borrow = current.as_mut();
                match current_borrow {
                    MerkleNode::Branch(branch) => {
                        queue.push_back(&mut branch.left);
                        queue.push_back(&mut branch.right);
                    }
                    MerkleNode::Leaf(leaf) => {
                        let mut new_leaf = MerkleNode::new_leaf(data, &self.hasher);

                        let hash = leaf
                            .as_ref()
                            .and_then(|node| {
                                let mut block_hasher = self.hasher.build_hasher();
                                node.block_hash.hash(&mut block_hasher);
                                new_leaf.get_hash().hash(&mut block_hasher);
                                Some(block_hasher.finish())
                            })
                            .unwrap_or(new_leaf.get_hash());

                        let mut new_branch = Box::new(MerkleNode::Branch(BranchNode {
                            left: Box::new(MerkleNode::Leaf(leaf.take())),
                            right: new_leaf,
                            hash,
                        }));

                        std::mem::swap(current_borrow, &mut new_branch);
                        return;
                    }
                }
            }
        } else {
            self.head.replace(MerkleNode::new_leaf(data, &self.hasher));
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

    #[test]
    fn put_stuff_in() {
        let mut tree: MerkleTree<u64, DefaultBuildHasher> = MerkleTree::new();
        for i in 0..5 {
            tree.push(i)
        }
        tree.debug()
    }
}
