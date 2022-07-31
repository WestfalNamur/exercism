use std::collections::HashMap;

fn get_invalid_nucleotides(dna: &str) -> Vec<char> {
    // If the string contains characters that aren't A, C, G, or T then it is
    // invalid and you should signal an error.
    // Notes:
    //  - Double-quotes "" create string literals, while single-quotes '' create
    //  character literals.
    //  - iterator.contains()
    dna.chars()
        .filter(|c| !['A', 'C', 'G', 'T'].contains(&c))
        .collect()
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    unimplemented!(
        "How much of nucleotide type '{}' is contained inside DNA string '{}'?",
        nucleotide,
        dna
    );
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    // Handle invalid chars
    let invalid_nucleotides: Vec<char> = get_invalid_nucleotides(dna);
    if invalid_nucleotides.len() > 0 {
        // [0] As only one chars is expected and not a vector of all wrong chars.
        return Err(invalid_nucleotides[0]);
    }

    unimplemented!(
        "How much of every nucleotide type is contained inside DNA string '{}'?",
        dna
    );
}
