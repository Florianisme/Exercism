#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.to_vec()
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut copy = self.scores.clone();
        copy.sort();
        let mut score_vec = vec![];
        for _x in 0..3 {
            match copy.pop() {
                Some(value) => score_vec.push(value),
                _ => {}
            }
        }
        score_vec
    }
}
