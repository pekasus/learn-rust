use std::collections::BTreeSet;

pub struct Triangle(BTreeSet<u64>);

impl Triangle {
    pub fn build(mut sides: [u64; 3]) -> Option<Triangle> {
        sides.sort();
        let is_any_zero: bool = sides[0] == 0;
        let valid_shape: bool = sides[0] + sides[1] >= sides[2];
        let sides_set = BTreeSet::from(sides);

        if !is_any_zero && valid_shape {
            Some(Triangle(sides_set))
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.0.len() == 1
    }

    pub fn is_scalene(&self) -> bool {
        self.0.len() == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.0.len() == 2
    }
}
