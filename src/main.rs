mod genome;
mod binarytree;
use genome::Expression;
use binarytree::BinaryTree;

fn main() {
    let mut root = BinaryTree::new(&5);
    root.insert(&10);
    println!("Binary Tree root: {}", root.val);

    let gene = Expression::new().sequence().evaulate(42.0);
    println!("Sequenced genome: {}, fitness: {}", &gene.sequenced.as_ref().unwrap(), &gene.fitness);
}
