use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    lookup_map: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        if let Some(name) = self.lookup_map.get(codon) {
            Some(*name)
        } else {
            None
        }
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let chunks_vec = rna.chars().collect::<Vec<_>>();
        let chunks = chunks_vec
            .chunks(3)
            .map(|chunk| chunk.iter().collect::<String>());

        let result: Option<(Vec<&'a str>, bool)> =
            chunks.fold(Some((Vec::new(), false)), |compound_option, chunk| {
                if let Some((vec, stop_flag)) = compound_option {
                    if stop_flag {
                        Some((vec, true))
                    } else {
                        let name = *self.lookup_map.get(chunk.as_str())?;
                        if name == "stop codon" {
                            Some((vec, true))
                        } else {
                            let mut vec = vec;
                            vec.push(name);
                            Some((vec, false))
                        }
                    }
                } else {
                    None
                }
            });

        result.and_then(|(vec, _)| Some(vec))
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut map = HashMap::new();
    for pair in pairs {
        map.insert(pair.0, pair.1);
    }
    CodonsInfo { lookup_map: map }
}
