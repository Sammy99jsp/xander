use std::{
    ops::{Index, IndexMut},
    sync::{RwLock, Weak},
};

use crate::{
    core::{
        geom::{resolve_distance, P3},
        stats::monster::speed::{Crawling, SpeedType, SpeedTypeMeta, Walking},
    },
    utils::legality::{self},
};

use super::Combatant;

#[derive(Debug)]
pub struct MovementCtx {
    pub(super) combatant: Weak<Combatant>,
    used: RwLock<UsedMovement>,
}

#[derive(Debug, Clone, Copy)]
pub enum Medium {
    Ground,
    Air,
    Water,
}

#[derive(Debug, Default, Clone)]
pub struct UsedMovement {
    walking: u32,
    burrowing: u32,
    climbing: u32,
    flying: u32,
    swimming: u32,
}

impl UsedMovement {
    pub fn sum(&self) -> u32 {
        self.walking + self.burrowing + self.climbing + self.flying + self.swimming
    }
}

impl Index<&'static SpeedTypeMeta> for UsedMovement {
    type Output = u32;

    fn index(&self, speed: &'static SpeedTypeMeta) -> &Self::Output {
        if speed.name == Crawling::META.name {
            return &0;
        }

        unsafe {
            (&raw const self as *const u32)
                .add(speed.index)
                .as_ref_unchecked()
        }
    }
}

impl IndexMut<&'static SpeedTypeMeta> for UsedMovement {
    fn index_mut(&mut self, speed: &'static SpeedTypeMeta) -> &mut Self::Output {
        if speed.name == Crawling::META.name {
            unimplemented!()
        }

        unsafe {
            (self as *mut UsedMovement as *mut u32)
                .add(speed.index)
                .as_mut_unchecked()
        }
    }
}

pub const CANNOT_MOVE_THERE: legality::Reason = legality::Reason {
    id: "CANNOT_MOVE_THERE",
};

pub const NOT_ENOUGH_MOVEMENT_LEFT: legality::Reason = legality::Reason {
    id: "NOT_ENOUGH_MOVEMENT_LEFT",
};

pub const CANNOT_USE_MODE: legality::Reason = legality::Reason {
    id: "CANNOT_USE_MODE",
};

impl MovementCtx {
    pub fn new(combatant: Weak<Combatant>) -> Self {
        Self {
            combatant,
            used: Default::default(),
        }
    }

    pub fn used(&self) -> u32 {
        self.used.read().unwrap().sum()
    }

    pub fn any_movement_left(&self, mode: &'static SpeedTypeMeta) -> legality::Legality<()> {
        let combatant = self.combatant.upgrade().unwrap();

        match combatant.stats.speeds.of_type(mode) {
            None => legality::Legality::Illegal(CANNOT_USE_MODE),
            Some(speed) if speed <= self.used() => {
                legality::Legality::Illegal(NOT_ENOUGH_MOVEMENT_LEFT)
            }
            Some(_) => legality::Legality::Legal(()),
        }
    }

    /// Try to move a distance with a certain mode (of speed).
    #[must_use]
    pub fn try_move(
        &self,
        mode: &'static SpeedTypeMeta,
        displacement: P3,
    ) -> legality::Legality<()> {
        use legality::Legality::*;
        let combatant = self.combatant.upgrade().unwrap();
        let combat = combatant.combat.upgrade().unwrap();
        let speeds = &combatant.stats.speeds;

        // TODO: Add other types of movement here.
        // TODO: Account for difficult terrain here.
        // TODO: Add the various checks for those too.

        if mode.index != Walking::META.index {
            todo!("{} is not supported yet", mode.name);
        }

        let distance = resolve_distance(&displacement);
        // We do not support non-integer distances yet.
        assert!(distance.fract() == 0.0);
        let distance = distance.floor() as u32;

        let total_used = self.used.read().unwrap().sum();
        let Some(can_move) = speeds
            .of_type(mode)
            .map(|speed| (speed - total_used) >= distance)
        else {
            return Illegal(CANNOT_USE_MODE);
        };

        if !can_move {
            return Illegal(NOT_ENOUGH_MOVEMENT_LEFT);
        }

        let old = combatant.position.get_cloned().unwrap();
        // TODO: There's probably a 'correct' way to do this
        // using [nalgebra]. But that's for later!
        let new = P3::new(
            old.x + displacement.x,
            old.y + displacement.y,
            old.z + displacement.z,
        );

        // Checks if we can actually move there,
        // otherwise early returns due to the `?`.
        let arena = combat.arena.as_ref();
        let () = arena.is_passable(new, combatant.stats.size)?;

        self.used.write().unwrap()[mode] += distance;
        combatant.position.replace(new).unwrap();

        // TODO: Apply any effects from arena.at(new),
        //       And remove any others.

        Legal(())
    }
}
