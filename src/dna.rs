/*
Counting DNA Nucleotides
https://rosalind.info/problems/dna/
*/

use std::str::Chars;

#[allow(non_snake_case)]
#[derive(PartialEq, Debug)]
struct NucleotideCounts {
    A: usize,
    C: usize,
    G: usize,
    T: usize,
}

impl NucleotideCounts {
    fn count(seq: Chars) -> Self {
        let mut counts = NucleotideCounts {A: 0, C: 0, G: 0, T: 0};
        for nucleotide in seq {
            match nucleotide {
                'A' => counts.A += 1,
                'C' => counts.C += 1,
                'G' => counts.G += 1,
                'T' => counts.T += 1,
                _ => continue
            }
        }
        counts
    }
}

#[cfg(test)]
mod tests {
    use crate::dna::NucleotideCounts;

    #[test]
    fn it_works() {
        let seq = "ACGTACGT".chars();
        let counts = NucleotideCounts::count(seq);
        assert_eq!(2, counts.A);
        assert_eq!(2, counts.C);
        assert_eq!(2, counts.G);
        assert_eq!(2, counts.T);
    }

    #[test]
    fn actual_dataset() {
        let dataset = "CCGAGACTAGGTTTTGAGCCCTGAGATAGAATAGGGTACAGAACCCCCTGGCATCGCGTAGTAGACCGATCTTTCACCAATTTGTCCGGTGTTAAGGCAGAAGAGTTAAACGTGTGCAAGAGGAAGTTCAATGGTGAAAAGCGAGTTAGTAAATGTTTGCATAAGAGTACTTGACTGAAGGGGACGGGCGTGCGACATAAGACACCGGACTACAACAATGAACACAAGGAGCCCTTATTTTTAATCACGTCACTTTCGAAATCTCGACAACCAAACGTGTACGAACTATTATCTACCGGTGGGGCGGCCGGTCCGGCCATAGCAGCCCGAATAAGCGAACCGGGGCCAAAAACGTCTAGATTCCCTTCGTGATGCGAAGGCCTGCGTCGTCGAAACACCCTGCGTTACCGTCCGTCCTTATAGGTGCCATCGCGACACCCAGATTGCCTACTTGTAACCGGCGAGCCGTGGTTACCTATAACGATAGATGCATACTTGTTGGCCGGAAGATTCCACATCAAGGGTACCCGGAACAGTCGCAACACAGACACGGTCATGGTATTACCAGATCTGACAGCACGCAACTCATCGCCGGGGTAAACATAGGCGGATTAAGTAGTCGGTACAGTCTGAACTGTGCGGGACGTGTTTGAGCTGCGTAGTACGCAATCGCCATTCCCACAGTCCCCAGCCGTCGGTAAGTATGGTCGAGACCCTCCCGGGGGATCGATGGATGGCGACCTCGCCGCACGTCTTGTCTTACCAAAATGTGCGCGACGTTCAGTCTGGCTCCGATGTAGTTCGCCTTTCCCTATAACTGAAGACTGTCCACTGTGCAATGTCCCGTCGGATCCAGCC";
        let counts = NucleotideCounts::count(dataset.chars());
        println!("{} {} {} {}", counts.A, counts.C, counts.G, counts.T);
        // Correct answer = 219 225 225 189
    }
}
