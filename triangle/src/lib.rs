use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::Add;

pub struct Triangle {
    t_type: TriangleType,
}

#[derive(PartialOrd, PartialEq)]
enum TriangleType {
    Equilateral,
    Isosceles,
    Scalene,
}

impl Triangle {
    pub fn build<T: Add<Output = T> + Ord + From<i32> + Copy + Hash>(
        sides: [T; 3],
    ) -> Option<Triangle> {
        let mut sorted_sides = sides.to_vec();
        sorted_sides.sort_by(|a, b| return a.partial_cmp(b).unwrap_or(Ordering::Equal));
        if ((sorted_sides[0] + sorted_sides[1]) < sorted_sides[2])
            || sorted_sides.iter().any(|side| *side <= T::from(0))
        {
            return None;
        }
        let distinct_side = HashSet::<&T>::from_iter(sorted_sides.iter()).len();
        let t_type = match distinct_side {
            1 => TriangleType::Equilateral,
            2 => TriangleType::Isosceles,
            _ => TriangleType::Scalene,
        };
        Some(Triangle { t_type })
    }

    pub fn is_equilateral(&self) -> bool {
        self.t_type == TriangleType::Equilateral
    }

    pub fn is_scalene(&self) -> bool {
        self.t_type == TriangleType::Scalene
    }

    pub fn is_isosceles(&self) -> bool {
        self.t_type == TriangleType::Isosceles
    }
}
