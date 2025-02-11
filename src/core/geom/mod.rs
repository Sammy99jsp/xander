//! # Geometry

pub mod shapes;

use std::fmt::Debug;

use crate::utils::{legality, reactive::Lifespan};

use super::cause::Cause;
use nalgebra::{Matrix3, Matrix4, Point3, Vector3, Vector4};

/// All coordinates are measured in feet.
pub type Coord = f32;

/// A point in 3D space.
pub type P3 = Point3<Coord>;

/// 3D vector.
pub type V3 = Vector3<Coord>;

/// 4D vector (for homogenous 3D)
pub type V4 = Vector4<Coord>;

/// 3x3 Matrix
pub type M3 = Matrix3<Coord>;

/// 4x4 Matrix (for homogenous 3x3)
pub type M4 = Matrix4<Coord>;

pub trait Shape: Debug + Send + Sync {
    fn contains(&self, p: P3) -> bool;
}

/// Area of Effect.
#[derive(Debug)]
pub struct AOE {
    cause: Lifespan<dyn Cause>,
    shape: Box<dyn Shape>,
}

impl AOE {
    pub fn contains(&self, p: P3) -> bool {
        self.shape.contains(p)
    }
}

pub fn resolve_distance(delta: &P3) -> f32 {
    omni_distance(delta)
}

/// Finds the (minimal) distance of a move using the naive
/// rules implied by 5.1E SRD's pg. 92.
///
/// In theory, this does break geometry, but *shrug*.
fn omni_distance(delta: &P3) -> f32 {
    const fn sgn(x: f32) -> f32 {
        if x < 0.0 {
            -1.0
        } else {
            1.0
        }
    }

    // This algorithm does not generalize to 3D yet.
    assert!(delta.z == 0.0);

    let c = f32::min(delta.x.abs(), delta.y.abs());
    let a = sgn(delta.x) * (delta.x - sgn(delta.x) * c);
    let b = sgn(delta.y) * (delta.y - sgn(delta.y) * c);

    a + b + c
}

pub const CANNOT_FIT: legality::Reason = legality::Reason { id: "CANNOT_FIT" };

#[cfg(test)]
mod tests {
    use crate::core::geom::P3;

    use super::resolve_distance;

    #[test]
    fn test_omni() {
        // In all four quadrants...
        assert_eq!(2.0, resolve_distance(&P3::new(1.0, 2.0, 0.0)));
        assert_eq!(3.0, resolve_distance(&P3::new(-3.0, 1.0, 0.0)));
        assert_eq!(3.0, resolve_distance(&P3::new(3.0, -2.0, 0.0)));
        assert_eq!(3.0, resolve_distance(&P3::new(-3.0, -3.0, 0.0)));
    }
}
