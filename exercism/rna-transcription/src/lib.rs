#[derive(Debug, PartialEq)]
pub struct Dna {
    strand: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    strand: String,
}

use std::collections::HashMap;

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let nucleotides = vec!['G', 'C', 'T', 'A'];
        let illegal_nucleotide_index = dna.chars().enumerate().find_map(|(idx, c)| {
            if !nucleotides.iter().any(|n| n == &c) {
                Some(idx)
            } else {
                None
            }
        });
        if let Some(idx) = illegal_nucleotide_index {
            Err(idx)
        } else {
            Ok(Dna {
                strand: dna.to_owned(),
            })
        }
    }

    pub fn into_rna(self) -> Rna {
        let mapping = HashMap::from([('G', 'C'), ('C', 'G'), ('T', 'A'), ('A', 'U')]);
        let rna_strand: String = self.strand.chars().map(|c| mapping[&c]).collect();
        Rna { strand: rna_strand }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let nucleotides = vec!['G', 'C', 'U', 'A'];
        let illegal_nucleotide_index = rna.chars().enumerate().find_map(|(idx, c)| {
            if !nucleotides.iter().any(|n| n == &c) {
                Some(idx)
            } else {
                None
            }
        });
        if let Some(idx) = illegal_nucleotide_index {
            Err(idx)
        } else {
            Ok(Rna {
                strand: rna.to_owned(),
            })
        }
    }
}
