use std::array;

use crate::core::geom::{Shape, M4, P3, V3, V4};

pub type Cube = Parallepiped;

#[derive(Debug, Clone, Copy)]
pub struct Parallepiped {
    origin: P3,
    sides: [V3; 3],
    inv_matrix: M4,
}

impl Parallepiped {
    pub fn new(origin: P3, sides: [V3; 3]) -> Self {
        let cols: [V4; 4] = array::from_fn(|i| match i {
            i @ ..3 => sides[i].to_homogeneous(),
            3 => origin.to_homogeneous(),
            _ => unreachable!(),
        });

        Self {
            origin,
            sides,
            inv_matrix: M4::from_columns(&cols)
                .try_inverse()
                .expect("Invalid cube. Singular matrix from edges!"),
        }
    }
}

impl Shape for Parallepiped {
    fn contains(&self, p: P3) -> bool {
        let p_trans = self.inv_matrix.transform_point(&p);

        (0.0..=1.0).contains(&p_trans.x)
            & (0.0..=1.0).contains(&p_trans.y)
            & (0.0..=1.0).contains(&p_trans.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::geom::{Shape, P3, V3};

    use super::Parallepiped;

    #[test]
    fn test_cube_intersection() {
        let cube = Parallepiped::new(P3::origin(), [2.0 * V3::x(), 2.0 * V3::y(), 2.0 * V3::z()]);
        assert!(cube.contains(P3::new(1.5, 0.5, 0.5)));
        assert!(!cube.contains(P3::new(2.5, 0.5, 0.5)));

        let cube = Parallepiped::new(
            P3::origin(),
            [
                V3::new(0.8365, 0.525, -0.158),
                V3::new(-0.224, 0.592, 0.775),
                V3::new(0.5, -0.612, 0.612),
            ],
        );

        assert!(cube.contains(P3::new(0.5, 0.5, 0.5)));
    }
}
