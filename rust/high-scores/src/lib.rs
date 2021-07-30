#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            // Copies self into a new Vec. Thus we have the scores vector 2x on
            // the heap. Hint: A Vec indicates ownership of the memory, and a
            // slice indicates a borrow of memory.
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        // return a reference to the scores of the current instance.
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        // Get the last and pass a copy of it.
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        // Iterate over elements, get the max and pass a copy.
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        // copy, sort, cutoff and return scores.
        let mut scores_copy = self.scores.to_vec();
        scores_copy.sort_unstable_by(|a, b| b.cmp(a));
        scores_copy.truncate(3);
        scores_copy
    }
}
