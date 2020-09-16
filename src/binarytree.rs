/// # Binary Tree implementation using traits
///
/// A binary tree holds a reference to some data and two child nodes, which can be `None` or
/// the root of a subtree.
pub struct BinaryTree<'a, T> {
    pub val: &'a T,
    pub left: Option<Box<BinaryTree<'a, T>>>,
    pub right: Option<Box<BinaryTree<'a, T>>>,
}

impl<'a, T: PartialOrd + std::fmt::Debug> BinaryTree<'a, T> {
    pub fn new(new_val: &'a T) -> Self {
        BinaryTree {
            val: new_val,
            left: None,
            right: None,
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, new_val: &'a T) {
        let target = if new_val < self.val {
            &mut self.left
        } else {
            &mut self.right
        };
        match target {
            &mut Some(ref mut subtree) => subtree.insert(new_val),
            &mut None => {
                let new_binary_tree = BinaryTree::new(new_val);
                let boxed_binary_tree = Some(Box::new(new_binary_tree));
                *target = boxed_binary_tree;
            }
        }
    }

    #[allow(dead_code)]
    pub fn print(&self, level: u64) {
        println!("level {}: {:?}", level, self.val);
        if self.left.is_some() {
            self.left.as_ref().unwrap().print(level + 1);
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().print(level + 1);
        }
    }
}
