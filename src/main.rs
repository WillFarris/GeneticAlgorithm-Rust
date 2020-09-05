mod genome;
use genome::Genome;

/// Binary Tree implementation using traits
/// 
/// A binary tree holds a reference to some data and two child nodes, which can be `None` or
/// the root of a subtree.
struct BinaryTree<'a, T> {
    val: &'a T,
    left: Option<Box<BinaryTree<'a, T>>>,
    right: Option<Box<BinaryTree<'a, T>>>,
}

impl<'a, T: PartialOrd> BinaryTree<'a, T> {
    fn new(new_val: &'a T) -> BinaryTree<T> {
        BinaryTree {
            val: new_val,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, new_val: &'a T) {
        let target = if new_val < self.val {&mut self.left} else {&mut self.right};
        match target {
            &mut Some(ref mut subtree) => subtree.insert(new_val),
            &mut None => {
                let new_binary_tree = BinaryTree::new(new_val);
                let boxed_binary_tree = Some(Box::new(new_binary_tree));
                *target = boxed_binary_tree;
            }
        }
    }
}

fn main() {
    let mut root = BinaryTree::new(&5);
    root.insert(&10);
    println!("Binary Tree: {} -> {}", root.val, root.right.unwrap().val);

    let mut gene = Genome::new();
    gene.sequence();
    println!("Genome: {}", gene.sequenced.unwrap());
}
