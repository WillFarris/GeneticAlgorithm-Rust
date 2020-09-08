use crate::binarytree::BinaryTree;

pub struct Expression {
    pub genome: u64,
    pub fitness: f64,
    pub valid_genes: Vec<u8>,
    pub sequenced: Option<String>,
}

impl Expression {
    pub fn new() -> Self {
        let genome = rand::random::<u64>();
        let fitness = 0.0;
        Expression {
            genome,
            fitness,
            valid_genes: Vec::with_capacity(16),
            sequenced: None,
        }
    }

    pub fn sequence(mut self) -> Self {
        if self.sequenced != None {
            return self;
        }
        
        let mut temp = String::with_capacity(16);
        
        let mut prev = 0xF;
        for i in 0..16 {
            let cur = ((self.genome >> i*4) & 0xF) as u8;
            
            if (cur < 0xA && prev > 0x9) || (cur > 0x9 && prev < 0xA) && cur < 0xE {
                let cur_char = match cur {
                    0xA => '*',
                    0xB => '/',
                    0xC => '+',
                    0xD => '-',
                    0xE..=0xF => '#', // ignored
                    _   => (cur + 48) as char, //48 == ascii '0'
                };

                temp.push(cur_char);
                self.valid_genes.push(cur);
                prev = cur;
            }
        }

        // for lack of time to write a better algorithm
        let last = self.valid_genes.last().unwrap(); //temp.chars().last().unwrap();
        if !(*last <= 9) {
            temp.pop();
            self.valid_genes.pop();
        }

        self.sequenced = Some(temp);
        self
    }

    pub fn evaulate(mut self, target: f64) -> Self {
        let symbols = &self.valid_genes;
        let root = build_tree(symbols);
        let result = sum_tree(&root);
        println!("Expression evaluates to: {}", result);
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
        return None
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
        right: build_tree(&subarray[last_op_index+1..subarray.len()])
    }))
}