use std::collections::HashMap;

const NUCLEOTIDE: &[char] = &['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDE.contains(&nucleotide) {
        return Err(nucleotide);
    }
    Ok(*nucleotide_counts(dna)?.get(&nucleotide).unwrap())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut nucleotides: HashMap<char, usize> = HashMap::new();
    NUCLEOTIDE.iter().for_each(|x| {
        nucleotides.insert(*x, 0);
    });
    for x in dna.trim().chars() {
        if !NUCLEOTIDE.contains(&x) {
            return Err(x);
        } else if let Some(count) = nucleotides.get_mut(&x) {
            *count += 1
        }
    }
    Ok(nucleotides)
}
