#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a[u32],   // stores a reference to a slice of u32.
                        // The 'a lifetime parameter indicates that the HighScores struct
                        // cannot outlive the slice it references.
                        // This is needed to make sure the data is not dropped 
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores {
            scores
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_scores: Vec<u32> = self.scores.to_vec();
        top_scores.sort_unstable_by(|a, b| b.cmp(a));
        top_scores.truncate(3);
        top_scores
    }
}
