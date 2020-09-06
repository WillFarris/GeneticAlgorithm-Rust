mod genome;
mod binarytree;
use genome::Genome;
use binarytree::BinaryTree;



fn main() {
    let mut root = BinaryTree::new(&5);
    root.insert(&10);
    println!("Binary Tree root: {}", root.val);

    let mut gene = Genome::new();
    gene.sequence();
    println!("Genome: {}, fitness: {}", &gene.sequenced.as_ref().unwrap(), &gene.fitness);
    gene.evaulate();
}
