mod binarytree;
mod genome;
use genome::Expression;
fn main() {
    let gene = Expression::new().sequence().evaulate(42.0);
    println!(
        "Sequenced genome: {}={}, fitness: {}",
        &gene.expression(),
        &gene.result,
        &gene.fitness
    );
}
