pub struct Genome {
    gene: u64,
    fitness: f64,
    pub sequenced: Option<String>,
}

impl Genome {
    pub fn new() -> Genome {
        let new_gene = rand::random::<u64>();
        let fitness = 0.0;
        Genome {
            gene: new_gene,
            fitness: fitness,
            sequenced: None,
        }
    }

    pub fn sequence(&mut self) {
        if self.sequenced != None {
            return
        }

        let mut temp: String = String::with_capacity(16);
        for i in 0..16 {
            let cur = ((self.gene >> i*4) & 0xF) as u8;
            match cur {
                    0xA => temp.push('+'),
                    0xB => temp.push('-'),
                    0xC => temp.push('*'),
                    0xD => temp.push('/'),
                    0xE..=0xF => (),
                    _   => temp.push((cur + 48) as char), //48 == ascii 0
            }
        }
        self.sequenced = Some(temp);
    }
}