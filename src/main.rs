mod genome;
mod binarytree;
use genome::Expression;

fn main() {
    let gene = Expression::new().sequence().evaulate(42.0);
    println!("Sequenced genome: {}={}, fitness: {}", &gene.sequenced.as_ref().unwrap(), &gene.result, &gene.fitness);
}
