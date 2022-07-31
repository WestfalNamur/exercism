use std::collections::HashMap;

const VALID_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

fn get_invalid_nucleotides_in_dna(dna: &str) -> Vec<char> {
    // If the string contains characters that aren't A, C, G, or T then it is
    // invalid and you should signal an error.
    // Notes:
    //  - Double-quotes "" create string literals, while single-quotes '' create
    //  character literals.
    //  - iterator.contains()
    dna.chars()
        .filter(|c| !VALID_NUCLEOTIDES.contains(&c))
        .collect()
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    // Handle empty dna
    if dna.len() == 0 {
        return Ok(0);
    }

    // Handle invalid nucleotide
    if !VALID_NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }

    // Handle invalid DNA
    let invalid_nucleotides_in_dna = get_invalid_nucleotides_in_dna(dna);
    if invalid_nucleotides_in_dna.len() > 0 {
        // [0] As only one chars is expected and not a vector of all wrong chars.
        return Err(invalid_nucleotides_in_dna[0]);
    }

    // Count given nucleotide
    // TODO: In one line?
    let searched_nucleotides: Vec<char> = dna.chars().filter(|&c| c == nucleotide).collect();
    Ok(searched_nucleotides.len())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    // Handle invalid chars
    let invalid_nucleotides: Vec<char> = get_invalid_nucleotides_in_dna(dna);
    if invalid_nucleotides.len() > 0 {
        // [0] As only one chars is expected and not a vector of all wrong chars.
        return Err(invalid_nucleotides[0]);
    }

    unimplemented!(
        "How much of every nucleotide type is contained inside DNA string '{}'?",
        dna
    );
}
