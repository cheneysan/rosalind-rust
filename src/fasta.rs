use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub type FastaMap = HashMap<String, String>;

pub fn load_fasta(path: &str) -> Result<FastaMap, std::io::Error> {
    let mut map = HashMap::new();
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();

    let mut label: Option<String> = None;
    let mut sequence: Option<String> = None;
    for line in lines.flatten() {
        if let Some(stripped) = line.strip_prefix('>') {
            if let Some(l) = label {
                map.insert(l, sequence.unwrap());
            }
            label = Some(String::from(stripped));
            sequence = None;
        } else if let Some(mut s) = sequence {
            s.push_str(&line);
            sequence = Some(s);
        } else {
            sequence = Some(line);
        }
    }

    if let Some(l) = label {
        map.insert(l, sequence.unwrap());
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use crate::fasta::load_fasta;

    #[test]
    fn it_works() {
        let path = "test/fasta.txt";
        let map = load_fasta(path);
        assert!(map.is_ok());
        let map = map.unwrap();

        assert_eq!("ACGTACGT", map.get("test-1").unwrap());
        assert_eq!("AAAAGGGCCTT", map.get("test-2").unwrap());
    }
}