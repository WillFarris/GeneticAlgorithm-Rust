use crate::binarytree::BinaryTree;

/// An `Expression` holds a 64-bit bitset representing its genome. 'fitness' holds the
pub struct Expression {
    genome: u64,
    pub fitness: f64,
    pub result: f64,
    valid_genes: Option<Vec<u8>>,
}

impl Expression {
    pub fn new() -> Self {
        let genome = rand::random::<u64>();
        let fitness = 0.0;
        Expression {
            genome,
            fitness,
            result: 0.0,
            valid_genes: None,
        }
    }

    pub fn sequence(mut self) -> Self {
        if self.valid_genes != None {
            return self;
        }

        let mut temp = Vec::with_capacity(16);

        let mut prev = 0xF;
        for i in 0..16 {
            let cur = ((self.genome >> i * 4) & 0xF) as u8;

            if (cur < 0xA && prev > 0x9) || (cur > 0x9 && prev < 0xA) && cur < 0xE {
                temp.push(cur);
                prev = cur;
            }
        }

        // TODO: Make the algorithm better so it doesn't need this hack
        let last = temp.last().unwrap();
        if !(*last <= 9) {
            temp.pop();
        }

        self.valid_genes = Some(temp);
        self
    }

    pub fn expression(&self) -> String {
        let mut result = String::with_capacity(16);
        for c in self.valid_genes.as_ref().unwrap().iter() {
            let cur_char: char = match c {
                0xA => '*',
                0xB => '/',
                0xC => '+',
                0xD => '-',
                0xE..=0xF => '#',      // ignored
                _ => (c + 48) as char, //48 == ascii '0'
            };
            result.push(cur_char);
        }
        result
    }

    pub fn evaulate(mut self, target: f64) -> Self {
        let symbols = &self.valid_genes;
        let root = build_tree(symbols.as_ref().unwrap());
        let result = sum_tree(&root);
        self.result = result;
        self.fitness = 1.0 / (target - result);
        self
    }
}

fn sum_tree(root: &Option<Box<BinaryTree<u8>>>) -> f64 {
    if root.is_none() {
        return 0.0;
    }

    let root_unwrapped = root.as_ref().unwrap();
    match *root_unwrapped.val {
        0..=9 => return *root_unwrapped.val as f64,
        10 => return (sum_tree(&root_unwrapped.left) * sum_tree(&root_unwrapped.right)) as f64,
        11 => return (sum_tree(&root_unwrapped.left) / sum_tree(&root_unwrapped.right)) as f64,
        12 => return (sum_tree(&root_unwrapped.left) + sum_tree(&root_unwrapped.right)) as f64,
        13 => return (sum_tree(&root_unwrapped.left) - sum_tree(&root_unwrapped.right)) as f64,
        _ => return 0.0,
    }
}

fn build_tree(subarray: &[u8]) -> Option<Box<BinaryTree<u8>>> {
    if subarray.len() <= 0 {
        return None;
    }
    let mut last_op_index = 0;
    for i in (0..subarray.len()).rev() {
        if subarray[i] > subarray[last_op_index] {
            last_op_index = i;
        }
    }

    Some(Box::new(BinaryTree {
        val: &subarray[last_op_index],
        left: build_tree(&subarray[0..last_op_index]),
        right: build_tree(&subarray[last_op_index + 1..subarray.len()]),
    }))
}
