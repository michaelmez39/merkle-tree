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
    data: T,
}

impl<T> LeafNode<T> {
    fn new<S>(data: T, hasher: &S) -> Self
    where
        S: BuildHasher,
        T: Hash,
    {
        let mut hasher = hasher.build_hasher();
        data.hash(&mut hasher);
        LeafNode {
            data,
            block_hash: hasher.finish(),
        }
    }
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
            data,
            block_hash: hasher.finish(),
        })))
    }

    fn new_branch<S>(left: LeafNode<T>, right: LeafNode<T>, build_hasher: &S) -> MerkleNode<T>
    where
        T: Hash,
        S: BuildHasher,
    {
        let mut hasher = build_hasher.build_hasher();
        left.block_hash.hash(&mut hasher);
        right.block_hash.hash(&mut hasher);
        let hash = hasher.finish();
        MerkleNode::Branch(BranchNode {
            hash,
            left: Box::new(MerkleNode::Leaf(Some(left))),
            right: Box::new(MerkleNode::Leaf(Some(right))),
        })
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
            head: None,
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
        let Some(head) = self.head.as_mut() else {
            self.head.replace(MerkleNode::new_leaf(data, &self.hasher));
            return;
        };

        let mut queue = VecDeque::from([head]);
        while let Some(current) = queue.pop_front() {
            let tree_cursor = current.as_mut();
            match tree_cursor {
                MerkleNode::Branch(branch) => {
                    queue.push_back(&mut branch.left);
                    queue.push_back(&mut branch.right);
                }
                MerkleNode::Leaf(leaf) => {
                    let left = leaf.take().expect("None found for leaf");
                    let right = LeafNode::new(data, &self.hasher);
                    let new_branch = MerkleNode::new_branch(left, right, &self.hasher);

                    let _ = std::mem::replace(tree_cursor, new_branch);
                    return;
                }
            }
            // TODO: Recalculate relevant tree hashes
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn empty_same_node() {
    //     let tree: MerkleTree<u64, BuildHasherDefault<DefaultHasher>> = MerkleTree::new();
    //     assert_eq!(tree.head, None);
    // }

    // #[test]
    // fn empty_same_hash() {
    //     let tree: MerkleTree<u64, DefaultBuildHasher> = MerkleTree::new();
    //     let mut hasher_tree = tree.hasher.build_hasher();
    //     let mut hasher_default = DefaultBuildHasher::default().build_hasher();
    //     let message = b"hello";

    //     hasher_tree.write(message);
    //     hasher_default.write(message);
    //     assert_eq!(hasher_default.finish(), hasher_tree.finish())
    // }

    #[test]
    fn put_stuff_in() {
        let mut tree: MerkleTree<u64, DefaultBuildHasher> = MerkleTree::new();
        for i in 0..3 {
            tree.push(i);
            println!("Adding: {}", i);
            tree.debug();
            println!();
        }
    }
}
