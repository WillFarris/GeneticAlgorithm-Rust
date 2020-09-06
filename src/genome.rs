use crate::binarytree::BinaryTree;

pub struct Genome {
    pub gene: u64,
    pub fitness: f64,
    pub sequenced: Option<String>,
}

impl Genome {
    pub fn new() -> Genome {
        let gene = rand::random::<u64>();
        let fitness = 0.0;
        Genome {
            gene,
            fitness,
            sequenced: None,
        }
    }

    pub fn sequence(&mut self) {
        if self.sequenced != None {
            return
        }
        
        let mut temp = String::with_capacity(16);
        
        let mut prev_raw = 0xF;
        for i in 0..16 {
            let cur_raw = ((self.gene >> i*4) & 0xF) as u8;
            
            if (cur_raw < 0xA && prev_raw > 0x9) || (cur_raw > 0x9 && prev_raw < 0xA) && cur_raw < 0xE {
                let cur_char = match cur_raw {
                    0xA => '*',
                    0xB => '/',
                    0xC => '+',
                    0xD => '-',
                    0xE..=0xF => '#', // ignored
                    _   => (cur_raw + 48) as char, //48 == ascii '0'
                };
                temp.push(cur_char);
                prev_raw = cur_raw;
            }
        }

        /*let mut last_was_op = true;
        let mut last = 0xF;
        let mut index = 0;
        while index < 16 {
            let mut cur = (self.gene >> i*4) & 0xF;
            let is_op = cur > 9;
            while last_was_op && cur > 9 {
                index += 1;
                cur = (self.gene >> i*4) & 0xF;
            }
            last_was_op = false;
            last = cur;
            let cur_char = match cur_raw {
                0xA => '*',
                0xB => '/',
                0xC => '+',
                0xD => '-',
                0xE..=0xF => '#', // ignored
                _   => (cur_raw + 48) as char, //48 == ascii '0'
            };
            temp.push(cur_char);
        }*/

        self.sequenced = Some(temp);
    }

    pub fn evaulate(&self) -> f64 {
        let mut members: [u8; 16] = [0xF; 16];
        for i in 0..16 {
            let current = ((self.gene >> (i*4)) & 0xF) as u8;
            members[i] = current;
        }
        println!("{:?}", members);
        //let root = build_tree(&members).unwrap();
        //root.print(0);
        0.0
    }
}

#[allow(dead_code)]
fn build_tree(subarray: &[u8]) -> Option<Box<BinaryTree<u8>>> {
    if subarray.len() <= 0 {
        return None
    }
    let mut last_op_index = 0;
    for i in subarray.len()..=0 {
        if subarray[last_op_index] > subarray[i] {
            last_op_index = i;
        }
    }

    Some(Box::new(BinaryTree {
        val: &subarray[last_op_index],
        left: build_tree(&subarray[0..last_op_index]),
        right: build_tree(&subarray[last_op_index..subarray.len()])
    }))
}