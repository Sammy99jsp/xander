//! Combat arena.

use std::sync::{Arc, Weak};

use crate::{
    core::{
        geom::{Coord, AOE, P3},
        stats::stat_block::Size,
    },
    utils::legality::{self, Legality, ThenLegal},
};

use super::{Combat, Combatant};

#[derive(Debug)]
pub struct Square<'a> {
    pub combatants: Vec<Arc<Combatant>>,
    pub effects: Vec<&'a AOE>,
}

pub const SQUARE_LENGTH: Coord = 5.0;

/// Round to the nearest grid square.
pub fn grid_round(a: Coord) -> f32 {
    (a / SQUARE_LENGTH).round() * SQUARE_LENGTH
}
pub fn grid_round_p(p: P3) -> P3 {
    let mut tmp = p / SQUARE_LENGTH;
    tmp.apply(|a| *a = a.round());
    tmp * SQUARE_LENGTH
}

pub trait ArenaConstructor {
    type Params;

    fn new(weak: Weak<Combat>, params: Self::Params) -> Self
    where
        Self: Sized;
}

pub trait Arena: Send + Sync {
    /// What's in the square at this location?
    fn at(&self, point: P3) -> Square;

    /// Can a combatant of some [Size] pass into this square?
    fn is_passable(&self, point: P3, size: Size) -> Legality<()>;

    #[cfg(feature = "vis")]
    fn visualize(&self) -> crate::vis::Image;
}

#[derive(Debug, Clone)]
pub struct SimpleArenaParams {
    dimensions: (Coord, Coord),
}

impl SimpleArenaParams {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            dimensions: (width, height),
        }
    }
}

/// Simple arena spanning from (0, 0) to (size.0, size.1).
///
/// This arena cannot have any obstacles, difficult terrain, etc.
/// -- it only has walls on its edges.
#[derive(Debug)]
pub struct SimpleArena {
    weak: Weak<Combat>,
    effects: Vec<AOE>,
    params: SimpleArenaParams,
}

impl SimpleArena {
    pub fn new(weak: Weak<Combat>, params: SimpleArenaParams) -> Self {
        Self {
            weak,
            params,
            effects: Default::default(),
        }
    }
}

impl std::ops::Deref for SimpleArena {
    type Target = SimpleArenaParams;

    fn deref(&self) -> &Self::Target {
        &self.params
    }
}

impl Arena for SimpleArena {
    fn at(&self, point: P3) -> Square {
        let grid_p = grid_round_p(point);
        Square {
            effects: self
                .effects
                .iter()
                .filter(move |aoe| aoe.contains(point))
                .collect::<Vec<_>>(),
            combatants: self
                .weak
                .upgrade()
                .unwrap()
                .initiative
                .as_vec()
                .into_iter()
                .filter(|a| grid_round_p(*a.position.read().expect("Not poisoned")) == grid_p)
                .collect(),
        }
    }

    #[allow(unused)]
    fn is_passable(&self, point: P3, size: Size) -> Legality<()> {
        let in_arena = (0.0..self.dimensions.0).contains(&grid_round(point.x))
            && (0.0..self.dimensions.1).contains(&grid_round(point.y));

        if !in_arena {
            return Legality::Illegal(OUT_OF_BOUNDS);
        }

        let Square { combatants, .. } = self.at(point);

        // TODO: Technically, allies can let a combatant through.
        // SRD: "If a Medium hobgoblin stands in a 5-foot-­wide
        //      doorway, other creatures can’t get
        //      through unless the hobgoblin lets them."
        // TODO: Many exceptions to that above rule.

        // TODO: Size check with surrounding squares.

        combatants.is_empty().then_legal_or(SPACE_OCCUPIED)
    }

    #[cfg(feature = "vis")]
    fn visualize(&self) -> crate::vis::Image {
        use crate::vis::{
            color::{rgba, TOKEN_COLORS},
            FindReplace, Grid, Root, Token,
        };

        const IMG: &[u8] = include_bytes!("../../../tests/python/img/rat.png");

        // SAFETY: IMG is &'static [u8], so it will outlast us all.
        let rat =
            skia::Image::from_encoded(unsafe { skia::Data::new_bytes(IMG) }).expect("Load rat!");

        let width = self.dimensions.0 as u32;
        let height = self.dimensions.1 as u32;

        let scale = 25;
        let mut root = Root::new(width, height, scale, 3);
        let c = self.weak.upgrade().expect("Not dead");

        root.add_child(Grid {
            square_len: 5,
            width,
            height,
            tokens: {
                c.initiative
                    .as_vec()
                    .into_iter()
                    .zip(TOKEN_COLORS.iter().copied().cycle())
                    .map(|(combatant, color)| {
                        Token::new(
                            *combatant.position.read().unwrap(),
                            rat.clone(),
                            FindReplace {
                                find: rgba(0x000000FF),
                                replace: color,
                            },
                        )
                    })
                    .collect()
            },
        });

        root.render()
    }
}

pub const OUT_OF_BOUNDS: legality::Reason = legality::Reason {
    id: "OUT_OF_BOUNDS",
};

pub const SPACE_OCCUPIED: legality::Reason = legality::Reason {
    id: "SPACE_OCCUPIED",
};
