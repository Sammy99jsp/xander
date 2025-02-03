//! ## Health
//!
//! This module concerns HP, Temporary HP, Max HP, and Hit Dice.

pub mod conditions;

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    fmt::Debug,
    rc::{Rc, Weak},
};

use conditions::{Condition, ConditionApplication, ConditionStatus, Unconscious};
use serde::Deserialize;

use crate::{
    core::{
        cause::Cause,
        dice::{DEvalTree, DExpr, Die},
    },
    utils::{
        reactive::{Ephemeral, Lifespan, RList, RSlot},
        Proxy,
    },
};

use super::{
    damage::{Damage, DamagePart},
    stat_block::{Dead, StatBlock},
};

type Value = u32;

#[derive(Debug)]
#[non_exhaustive]
pub struct Health {
    pub weak: Weak<StatBlock>,
    pub hp: HP,
    pub conditions: RefCell<ConditionStatus>,
    pub hit_dice: HitDice,
}

impl Health {
    pub fn new(this: &Weak<StatBlock>, max_hp: Proxy<StatBlock, u32>) -> Self {
        Health {
            weak: this.clone(),
            hp: HP::new(this.clone(), max_hp),
            conditions: RefCell::new(ConditionStatus::new(this.clone())),
            hit_dice: HitDice::new(),
        }
    }

    pub fn current_hp(&self) -> u32 {
        self.hp.current.get()
    }

    pub fn max_hp(&self) -> u32 {
        self.hp.max.get()
    }

    pub fn apply_condition(&self, cond: ConditionApplication) -> ConditionApplicationResult {
        let this = self.weak.upgrade().expect("Stat block present");
        match this.condition_immunities.is_immune(cond.condition) {
            r @ ConditionApplicationResult::Successful => {
                let mut conditions = self.conditions.borrow_mut();
                conditions.apply(cond);
                r
            }
            ConditionApplicationResult::Unsuccessful(cause) => {
                ConditionApplicationResult::Unsuccessful(cause)
            }
        }
    }

    pub fn take_damage(&self, damage: Damage) -> DamageTaken {
        let this = self.weak.upgrade().expect("Stat block present");
        let taken = this.damage_effectors.calculate(damage);

        let to_take: i32 = taken
            .0
            .iter()
            .map(|DamagePart { amount, .. }| amount.as_ref())
            .map(DEvalTree::result)
            .sum();

        let result = self.hp.damage(to_take.max(0).try_into().unwrap());

        DamageTaken {
            who: self.weak.clone(),
            taken,
            result,
        }
    }
}

#[derive(Debug)]
pub struct DamageTaken {
    who: Weak<StatBlock>, // TODO: Change this to [Creature] or something.
    taken: Damage,
    result: DamageResult,
}

#[derive(Debug, Clone)]
pub enum ConditionApplicationResult {
    Successful,
    Unsuccessful(Lifespan<dyn Cause>),
}

///
/// Handles HP, max HP, temporary HP (via [TempHP]),
///
///
#[derive(Debug)]
pub struct HP {
    stat_block: Weak<StatBlock>,

    pub current: Cell<u32>,
    pub max: Proxy<StatBlock, u32>,
    pub temp: RefCell<RSlot<TempHP>>,
    pub death_saves: RefCell<Option<Rc<DeathSaves>>>,
}

impl HP {
    pub fn new(stats: Weak<StatBlock>, max: Proxy<StatBlock, u32>) -> Self {
        Self {
            stat_block: stats,
            current: Cell::new(0), // Avoid setting this before init!
            max,
            temp: Default::default(),
            death_saves: RefCell::new(None),
        }
    }

    fn damage(&self, mut damage: u32) -> DamageResult {
        // Handle any temporary hit points.
        let mut temp_hp = self.temp.borrow_mut();
        match temp_hp.get_mut() {
            None => (),
            Some(TempHP { amount: tmp_hp, .. }) if *tmp_hp > damage => {
                *tmp_hp -= damage;
                damage = 0;
            }
            // if temp_hp <= damage...
            Some(_) => {
                // Negate the damage with whatever's left.
                // And remove the temp HP at the same time.
                damage -= temp_hp.take().unwrap().amount;
            }
        }

        let current = self.current.get();

        // Calculate excess damage
        // if this entity is taken to 0 HP as a result
        // of this damage.
        let excess = if damage >= current {
            let excess = damage - current;
            self.current.set(0);
            Some(excess)
        } else {
            self.current.update(|current| current - damage);
            None
        };

        match excess {
            None => DamageResult::Nothing,

            // SRD:
            // "When damage reduces you to 0 hit points and there is damage
            // remaining, you die if the remaining damage equals
            // or exceeds your hit point maximum."
            Some(dmg) if dmg >= self.max.get() => {
                // Declare ourselves dead.
                let creature = self.stat_block.upgrade().unwrap();
                creature.dead.set(Some(Dead));
                DamageResult::Death
            }

            // SRD:
            // "If damage reduces you to 0 hit points and fails to kill
            // you, you fall unconscious (see appendix PH-­A). This
            // unconsciousness ends if you regain any hit points."
            Some(_) => {
                // Start death saves.
                let creature = self.stat_block.upgrade().unwrap();
                let mut death_saves = self.death_saves.borrow_mut();
                *death_saves = Some(DeathSaves::new(creature.as_ref()));

                creature
                    .health
                    .conditions
                    .borrow_mut()
                    .apply(ConditionApplication {
                        lifespan: Lifespan::Indefinite,
                        condition: Unconscious::META,
                    });

                DamageResult::Unconscious
            }
        }
    }

    pub fn heal(&self, health: u32) {
        // If we are dead, do nothing.
        let creature = self.stat_block.upgrade().unwrap();
        if creature.dead.get() == Some(Dead) {
            return;
        }

        // Otherwise, stop the current death saves.
        let mut death_saves = self.death_saves.borrow_mut();
        if death_saves.is_some() {
            death_saves.take();
        }

        // SRD:
        // "Any hit points regained in excess of
        // [a creature's hit point maximum] are lost."
        self.current
            .update(|current| (current + health).min(self.max.get()));
    }

    pub fn temporary(&self, new: TempHP) {
        // SRD:
        // "[Temporary hit points] can't be added together.
        // If you have temporary hit points and receive more of them,
        // you decide whether to keep the ones you have or to gain the new ones. "
        let existing = self.temp.borrow_mut();

        if todo!("Decision!") {
            existing.replace(new);
        }
    }
}

#[derive(Debug)]
pub enum DamageResult {
    Nothing,
    Unconscious,
    Death,
}

///
/// Represents the temporary hit points
/// attached to an entity.
///
/// ---
///
/// SRD:
/// > "\[Temporary hit points\] are a buffer against damage, a
/// > pool of hit points that protect you from injury."
#[derive(Debug)]
pub struct TempHP {
    pub(crate) amount: Value,
    source: Option<Weak<dyn Cause>>,
}

impl Ephemeral for TempHP {
    fn is_alive(&self) -> bool {
        // The cause actually still exists.
        self.source.as_ref().is_some_and(|r| r.strong_count() > 0)
    }
}

#[derive(Deserialize)]
pub struct HitDice(RList<HitDie>);

impl Debug for HitDice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = self.0.list();
        let (used, available): (Vec<_>, Vec<_>) =
            list.iter().partition(|HitDie { used, .. }| *used);

        let used = histogram(used.into_iter().map(HitDie::die));
        let available = histogram(available.into_iter().map(HitDie::die));

        f.debug_struct("HitDice")
            .field("used", &used)
            .field("available", &available)
            .finish()
    }
}

#[derive(Debug)]
pub struct TooManyDice;

impl HitDice {
    pub const fn new() -> Self {
        Self(RList::new())
    }

    pub fn add(&mut self, hit_die: HitDie) {
        self.0.push_back(hit_die);
    }

    /// Get any available HitDice.
    pub fn available(&self) -> Vec<Die> {
        self.0
            .list()
            .iter()
            .filter(|die| !die.used)
            .map(|die| die.die)
            .collect()
    }

    pub fn use_dice(&mut self, to_take: Vec<Die>) -> Result<DExpr, TooManyDice> {
        let list = self.0.list_mut();

        let available = {
            let available = list
                .iter()
                .filter(|HitDie { used, .. }| !used)
                .map(|HitDie { die, .. }| die)
                .copied();

            histogram(available)
        };

        let mut to_take = histogram(to_take.into_iter());

        for (die, to_take) in to_take.iter() {
            match available.get(die) {
                Some(qty) if qty >= to_take => (),
                _ => return Err(TooManyDice),
            }
        }

        let mut expr: Option<DExpr> = None;

        for HitDie { die, used, .. } in list.iter_mut().filter(|HitDie { used, .. }| !used) {
            let Some(entry) = to_take.get_mut(die) else {
                continue;
            };

            if *entry > 0 {
                *used = true;
                *entry -= 1;

                let Some(ref mut expr) = expr else {
                    expr = Some((*die).into());
                    continue;
                };

                *expr += *die;
            }
        }

        Ok(expr.unwrap())
    }
}

fn histogram(iter: impl Iterator<Item = Die>) -> HashMap<Die, usize> {
    iter.fold(HashMap::new(), |mut map, die| {
        map.entry(die).and_modify(|dice| *dice += 1).or_insert(1);
        map
    })
}

impl Default for HitDice {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
#[serde(from = "Die")]
pub struct HitDie {
    source: Lifespan<dyn Cause>,
    die: Die,
    used: bool,
}

impl From<Die> for HitDie {
    fn from(die: Die) -> Self {
        Self {
            source: Lifespan::Indefinite,
            die,
            used: false,
        }
    }
}

impl Ephemeral for HitDie {
    fn is_alive(&self) -> bool {
        self.source.is_alive()
    }
}

impl HitDie {
    /// Create a new fixed die.
    pub fn fixed(die: Die) -> Self {
        Self {
            source: Lifespan::Indefinite,
            used: false,
            die,
        }
    }

    pub fn derived(source: &Rc<dyn Cause>, die: Die) -> Self {
        Self {
            source: Lifespan::of_this(source),
            die,
            used: false,
        }
    }

    pub fn die(&self) -> Die {
        self.die
    }
}

#[derive(Debug)]
pub struct DeathSaves {
    successes: u8,
    failures: u8,
}

impl DeathSaves {
    pub fn new(stat: &StatBlock) -> Rc<Self> {
        let this = Rc::new(Self {
            successes: 0,
            failures: 0,
        });

        let weak = Rc::downgrade(&this);
        todo!("Do action effects with Weak<Self> and the StatBlock");

        this
    }
}

#[derive(Debug)]
pub enum DeathSaveOutcome {
    Stabilized,
    Death,
}

impl DeathSaves {
    // pub fn success(&mut self) -> {}
}

impl std::fmt::Display for DeathSaves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeathSaves(S")?;

        for i in (0..3)
            .map(|i| i < self.successes)
            .map(|d| if d { "◈" } else { "◇" })
        {
            write!(f, "{i}")?;
        }

        write!(f, " ")?;

        for i in (0..3)
            .map(|i| i < self.failures)
            .map(|d| if d { "◈" } else { "◇" })
        {
            write!(f, "{i}")?;
        }

        write!(f, "F)")
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        core::{
            dice::{self, D20, D4},
            stats::{
                health::{TempHP, HP},
                stat_block::StatBlock,
                AbilityScore,
            },
        },
        utils::Proxy,
    };

    // #[test]
    // fn test_hit_dice() {
    //     fn s(raw: u8) -> AbilityScore {
    //         AbilityScore::new(raw).unwrap()
    //     }

    //     dice::random_seed();

    //     let rat = StatBlock::new(s(2), s(11), s(9), s(2), s(10), s(4), 3);
    // }
}
