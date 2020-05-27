pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut res: Vec<Vec<u32>> = vec![];
        for i in 0..self.row_count {
            let prev = if i == 0 {
                None
            } else {
                Some(&res[(i - 1) as usize])
            };
            let mut v: Vec<u32> = Vec::new();
            for j in 0..i + 1 {
                v.push(get_sum(prev, j as usize));
            }
            res.push(v);
        }
        res
    }
}

pub fn get_sum(vec: Option<&Vec<u32>>, n: usize) -> u32 {
    match vec {
        Some(v) => {
            if n == 0 || n == v.len() {
                1
            } else {
                v[n - 1] + v[n]
            }
        }
        None => 1,
    }
}
