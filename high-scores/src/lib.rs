#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
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
        let top_count: usize = if self.scores.len() < 3 {
            self.scores.len()
        } else {
            3
        };
        let mut top_vec: Vec<u32> = vec![0; top_count];
        'outer: for i in self.scores.iter() {
            for j in 0..top_count {
                if top_vec[j as usize] < *i {
                    top_vec[j..].rotate_right(1);
                    top_vec[j] = *i;
                    continue 'outer;
                }
            }
        }
        top_vec
    }
}
