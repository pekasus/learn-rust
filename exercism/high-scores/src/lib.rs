#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

fn option_ref_to_option(o: Option<&u32>) -> Option<u32> {
    match o {
        Some(x) => Some(*x),
        None => None,
    }
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        option_ref_to_option(self.scores.last())
    }

    pub fn personal_best(&self) -> Option<u32> {
        let option_ref = self.scores.iter().max();
        option_ref_to_option(option_ref)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        if self.scores.is_empty() {
            return vec![];
        }
        let mut sorted = self.scores.to_vec();
        sorted.sort();
        sorted.reverse();

        // 1 -> 1 == cmp::min(1, 3) == 3.min(1) == 1.min(3)
        // 4 -> 3 == cmp::min(4, 3) == 4.min(3) == 3.min(4)
        //let sorted_len = sorted.len().min(3);
        let sorted_len = if sorted.len() < 3 {
            sorted.len()
        } else { 
            3
        };
        sorted[0..sorted_len].to_vec()
    }
}
