/*
Finding a Motif in DNA
https://rosalind.info/problems/subs/
*/

fn find_motif_indices(dna: &str, motif: &str) -> Vec<usize> {
    let motif_chars = motif.chars().collect::<Vec<char>>();
    dna.chars().collect::<Vec<char>>()
        .windows(motif.len())
        .enumerate()
        .filter_map(|(i, chunk)| {
            if motif_chars == chunk {
                Some(i + 1)
            } else {
                None
            }
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use crate::subs::find_motif_indices;

    #[test]
    fn it_works() {
        let dna = "GATATATGCATATACTT";
        let motif = "ATAT";
        assert_eq!(vec!(2, 4, 10), find_motif_indices(dna, motif));
    }

    #[test]
    fn actual_dataset() {
        let dna = "CCTCCGCACCTCCGCCCTCCGCCCTCCGCGCGCCTCCGCCCTCCGCTAACGCCTCCCCCTCCGCTCCTCCGCCCTCCGCCCTCCGCACCTCCGCAGCACCTCCGCCCTCCGCAGCCTCCGCATTTCCGATCCTCCGCCCTCCGCGTGCCTCCGCGCCTGGGGCACCTCCGCACGACCTCCGCCTACCTCCGCGGCCTCCGCCCTCCGCTATCCTCCGCCACGACCCTGGCCTCCGCAGCCTCCGCCCTCCGCCCTCCGCAAGCCTCCGCAAGGCCTCCGCACCTCCGCACTGAGTCCTCCGCCACCTCCGCTACCTCCGCCCCTCCGCCCTCCGCACCTCCGCCCTCCGCTCCCCTCCGCCCTCCGCCCTCCGCCCTCCGCGGTGGTGAGAAGGTACCTCCGCATGCCTCCGCCCTCCGCCCTCCGCACCCCTCCGCTCCCCTCCGCCGACCTCCGCCCTCCGCTCCCTCCGCCGCCTCCGCCCTCCGCCCTCCGCCCTCCGCCCCACGGAGCCTCCGCCCTCCGCTTCCTCCGCTGACGCGCTGTTTTCCTCCGCAGGTCCTCCGCTTCCTCCGCGTTCCTCCGCGCCTCCGCGCACCTCCGCCCTCCGCGCCTCCGCTACCTCCGCGCCTCCGCTACCTCCGCCGCCTCCGCACAGCCCTCCGCCCACCATGCACCTCCGCTCCTACCTCCGCCCACGATCCTCCGCCCTCCGCCCTCCGCAATCCCCCTCCGCGTCCTCCGCCGCCTCCGCCCTCCGCCCTCCGCAGGCCCTCCGCCCTCCGCCTCCTCCGCCCTCCGCAGTGTCCCTCCGCACTATACCTCCGCTCGTCCTCCGCTCTTGGCCTCCGCTTCAACGCATCCTCCGCGAGGACACCCTCCGCTCACCTCCGCAGGTAGGCCTCCGCTGCGCGACAGGGTCACCTCCGCCCCTCCGC";
        let motif = "CCTCCGCCC";
        println!("{:?}", find_motif_indices(dna, motif));
    }
}