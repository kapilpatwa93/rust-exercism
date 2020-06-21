use std::slice::Iter;

#[derive(Debug, PartialEq)]
pub struct DNA {
    strands: Vec<DNANucleotide>,
}

impl Into<RNA> for DNA {
    fn into(self) -> RNA {
        let mut strands: Vec<RNANucleotide> = vec![];
        self.strands.iter().for_each(|&dna| {
            strands.push(dna.into());
        });
        RNA { strands }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum DNANucleotide {
    A,
    C,
    G,
    T,
}

impl DNANucleotide {
    pub fn iterator() -> Iter<'static, DNANucleotide> {
        [
            DNANucleotide::A,
            DNANucleotide::C,
            DNANucleotide::G,
            DNANucleotide::T,
        ]
        .iter()
    }
}

impl Into<RNANucleotide> for DNANucleotide {
    fn into(self) -> RNANucleotide {
        match self {
            DNANucleotide::A => RNANucleotide::U,
            DNANucleotide::C => RNANucleotide::G,
            DNANucleotide::G => RNANucleotide::C,
            DNANucleotide::T => RNANucleotide::A,
        }
    }
}

impl Into<char> for DNANucleotide {
    fn into(self) -> char {
        match self {
            DNANucleotide::A => 'A',
            DNANucleotide::C => 'C',
            DNANucleotide::G => 'G',
            DNANucleotide::T => 'T',
        }
    }
}

impl From<char> for DNANucleotide {
    fn from(strand: char) -> Self {
        match strand {
            'A' => DNANucleotide::A,
            'C' => DNANucleotide::C,
            'G' => DNANucleotide::G,
            'T' => DNANucleotide::T,
            _ => DNANucleotide::T,
        }
    }
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let mut strands: Vec<DNANucleotide> = vec![];
        for (i, new_nucleotide) in dna.chars().enumerate() {
            let a = DNANucleotide::iterator().any(|&nucleotide| {
                let c: char = nucleotide.into();
                c == new_nucleotide
            });
            if !a {
                return Err(i);
            }
            strands.push(DNANucleotide::from(new_nucleotide))
        }
        Ok(DNA { strands })
    }

    pub fn into_rna(self) -> RNA {
        self.into()
    }
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    strands: Vec<RNANucleotide>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum RNANucleotide {
    A,
    C,
    G,
    U,
}
impl RNANucleotide {
    pub fn iterator() -> Iter<'static, RNANucleotide> {
        [
            RNANucleotide::A,
            RNANucleotide::C,
            RNANucleotide::G,
            RNANucleotide::U,
        ]
        .iter()
    }
}
impl Into<char> for RNANucleotide {
    fn into(self) -> char {
        match self {
            RNANucleotide::A => 'A',
            RNANucleotide::C => 'C',
            RNANucleotide::G => 'G',
            RNANucleotide::U => 'U',
        }
    }
}

impl From<char> for RNANucleotide {
    fn from(strand: char) -> Self {
        match strand {
            'A' => RNANucleotide::A,
            'C' => RNANucleotide::C,
            'G' => RNANucleotide::G,
            'U' => RNANucleotide::U,
            _ => RNANucleotide::U,
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let mut strands: Vec<RNANucleotide> = vec![];
        for (i, new_nucleotide) in rna.chars().enumerate() {
            let a = RNANucleotide::iterator().any(|&nucleotide| {
                let c: char = nucleotide.into();
                c == new_nucleotide
            });
            if !a {
                return Err(i);
            }
            strands.push(RNANucleotide::from(new_nucleotide))
        }
        Ok(RNA { strands })
    }
}
