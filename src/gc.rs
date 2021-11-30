/*
Computing GC Content
https://rosalind.info/problems/gc/
*/

use crate::fasta::FastaMap;

fn highest_gc_content(fasta: &FastaMap) -> Option<(String, f64)> {
    let mut highest_label = None;
    let mut highest_gc = 0.0;

    fasta.iter().for_each(|(label, sequence)| {
        let gc_content = gc_content(sequence);
        if gc_content > highest_gc {
            highest_label = Some(label);
            highest_gc = gc_content;
        }
    });

    highest_label.map(|label| (label.clone(), highest_gc))
}

fn gc_content(sequence: &str) -> f64 {
    let mut count = 0.0;
    let mut gc = 0.0;
    sequence.chars().for_each(|c| {
        count += 1.0;
        if c == 'C' || c == 'G' {
            gc += 1.0;
        }
    });
    gc / count
}

#[cfg(test)]
mod tests {
    use crate::fasta::load_fasta;
    use crate::gc::highest_gc_content;

    #[test]
    fn it_works() {
        let fasta_map = load_fasta("test/gc.sample.txt").unwrap();
        println!("{:?}", fasta_map);
        let (label, gc) = highest_gc_content(&fasta_map).unwrap();
        assert_eq!("Rosalind_0808", label);
        assert_eq!("60.919540", format!("{:.6}", gc * 100.0));
    }

    #[test]
    fn actual_dataset() {
        let fasta_map = load_fasta("test/gc.dataset.txt").unwrap();
        let (label, gc) = highest_gc_content(&fasta_map).unwrap();
        println!("{}", label);
        println!("{:.6}", gc * 100.0);
        // Correct answer:
        // Rosalind_3210
        // 50.163221
    }
}