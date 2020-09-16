mod binarytree;
mod genome;
use genome::Expression;
//use std::thread;
//use std::sync::{Mutex, Arc};

fn main() {
    let target = 42.0;
    let population = {
        let mut temp = Vec::with_capacity(8);
        for _ in 0..8 {
            temp.push(Expression::new().sequence().evaulate(target));
        }
        temp.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
        temp
    };

    for i in population.iter() {
        println!(
            "Sequenced genome: {}={}, fitness: {}",
            &i.expression(),
            &i.result,
            &i.fitness
        );
    }
}
