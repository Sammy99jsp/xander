use crate::{
    core::dice::DEvalTree,
    utils::{meta::Meta, proxy::Dispatch, ProxyPart},
};
use paste::paste;
use std::{
    fmt::{Display, Formatter},
    ops::{Add, BitOr, Deref},
    ptr,
    sync::Weak,
    usize,
};

use super::stat_block::StatBlock;

#[derive(Debug)]
pub struct DamageTypeMeta {
    name: &'static str,
    doc: &'static str,
    index: usize,
}

impl DamageTypeMeta {
    pub const fn name(&self) -> &'static str {
        self.name
    }
    pub const fn doc(&self) -> &'static str {
        self.doc
    }
    pub(super) const fn index(&self) -> usize {
        self.index
    }
}

pub trait DamageType: Meta<DamageTypeMeta> {
    const META: &DamageTypeMeta;
}

macro_rules! damage_type {
    {$id: ident, doc: $doc: expr, index: $index: expr,} => {
        paste! {
            #[doc = $doc]
            #[derive(Debug, Clone, Copy)]
            pub struct $id;

            #[doc(hidden)]
            const [<META_ $id:snake:upper>] :&DamageTypeMeta = &DamageTypeMeta {
                name: stringify!([<$id:snake:upper>]),
                doc: $doc,
                index: $index,
            };

            impl DamageType for $id {
                const META: &DamageTypeMeta = [<META_ $id:snake:upper>];
            }

            impl Meta<DamageTypeMeta> for $id {
                fn meta(&self) -> &'static DamageTypeMeta {
                    [<META_ $id:snake:upper>]
                }
            }

            impl Deref for $id {
                type Target = DamageTypeMeta;

                fn deref(&self) -> &Self::Target {
                    Self::META
                }
            }
        }
    };
}

damage_type! {
    Acid,
    doc: "The corrosive spray of a black dragon's breath and the dissolving enzymes secreted by a black pudding deal acid damage.",
    index: 0,
}

damage_type! {
    Bludgeoning,
    doc: "Blunt force attacks—hammers, falling, constriction, and the like—deal bludgeoning damage.",
    index: 1,
}

damage_type! {
    Cold,
    doc: "The infernal chill radiating from an ice devil's spear and the frigid blast of a white dragon's breath deal cold damage.",
    index: 2,
}

damage_type! {
    Fire,
    doc: "Red dragons breathe fire, and many spells conjure flames to deal fire damage.",
    index: 3,
}

damage_type! {
    Force,
    doc: "Force is pure magical energy focused into a damaging form. Most effects that deal force damage are spells, including magic missile and spiritual weapon.",
    index: 4,
}

damage_type! {
    Lightning,
    doc: "A lightning bolt spell and a blue dragon's breath deal lightning damage.",
    index: 5,
}

damage_type! {
    Necrotic,
    doc: "Necrotic damage, dealt by certain undead and a spell such as chill touch, withers matter and even the soul.",
    index: 7,
}

damage_type! {
    Piercing,
    doc: "Puncturing and impaling attacks, including spears and monsters' bites, deal piercing damage.",
    index: 8,
}

damage_type! {
    Poison,
    doc: "Venomous stings and the toxic gas of a green dragon's breath deal poison damage.",
    index: 9,
}

damage_type! {
    Psychic,
    doc: "Mental abilities such as a mind flayer's psionic blast deal psychic damage.",
    index: 10,
}

damage_type! {
    Radiant,
    doc: "Radiant damage, dealt by a cleric's flame strike spell or an angel’s smiting weapon, sears the flesh like fire and overloads the spirit with power.",
    index: 11,
}

damage_type! {
    Slashing,
    doc: "Swords, axes, and monsters' claws deal slashing damage.",
    index: 12,
}

damage_type! {
    Thunder,
    doc: "A concussive burst of sound, such as the effect of the thunderwave spell, deals thunder damage.",
    index: 13,
}

/// Takes note of how this damage was previously handled
/// (e.g. resistance, vulnerability, immunity)
#[derive(Debug, Default, Clone)]
#[repr(align(16))] // TODO: Check if this is necessary to make the ProxyPart<..., DamageHandling> 64 bytes big.
pub struct DamageHandling {
    pub resistance: bool,
    pub vulnerability: bool,
    pub immunity: bool,
}

impl BitOr for DamageHandling {
    type Output = DamageHandling;

    fn bitor(
        self,
        Self {
            resistance,
            vulnerability,
            immunity,
        }: Self,
    ) -> Self::Output {
        Self {
            resistance: self.resistance | resistance,
            vulnerability: self.vulnerability | vulnerability,
            immunity: self.immunity | immunity,
        }
    }
}

#[derive(Debug)]
pub struct Resistance;

impl ProxyPart<StatBlock, DamageHandling> for Resistance {
    fn compute(&mut self, _: &StatBlock, prev: &mut DamageHandling, _: Dispatch<'_>) {
        prev.resistance = true;
    }
}

#[derive(Debug)]
pub struct Vulnerability;

impl ProxyPart<StatBlock, DamageHandling> for Vulnerability {
    fn compute(&mut self, _: &StatBlock, prev: &mut DamageHandling, _: Dispatch<'_>) {
        prev.vulnerability = true;
    }
}

#[derive(Debug)]
pub struct Immunity;

impl ProxyPart<StatBlock, DamageHandling> for Immunity {
    fn compute(&mut self, _: &StatBlock, prev: &mut DamageHandling, _: Dispatch<'_>) {
        prev.immunity = true;
    }
}

/* DAMAGE PROVENANCE */

#[derive(Debug)]
pub struct DamageCause {
    actor: DamageActor,
    source: DamageSource,
}

impl DamageCause {
    #[inline]
    pub fn is_magical(&self) -> bool {
        self.source.is_magical()
    }
}

/// Things that can cause damage.
#[derive(Debug)]
pub enum DamageActor {
    /// The environment itself.
    ///
    /// Damage from the DM is also included.
    Environment,
    Entity(Weak<()>), // TODO: Fix this type.
}

#[derive(Debug)]
pub struct DamageSource;

impl DamageSource {
    pub fn is_magical(&self) -> bool {
        todo!()
    }
}

/// Tracks an amount of damage, with a singular [DamageType],
/// with its [DamageCause]
#[derive(Debug)]
pub struct DamagePart {
    pub damage_type: &'static DamageTypeMeta,
    pub amount: Box<DEvalTree>,
    pub cause: DamageCause,
    pub handling: DamageHandling,
}

impl Display for DamagePart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.amount, self.damage_type.name)
    }
}

impl DamagePart {
    #[cfg(test)]
    pub fn environmental<D: DamageType>(_: D, amount: DEvalTree) -> Self {
        Self {
            damage_type: D::META,
            amount: Box::new(amount),
            cause: DamageCause {
                actor: DamageActor::Environment,
                source: DamageSource,
            },
            handling: Default::default(),
        }
    }
}

/// Multiple combinations of [DamagePart], making up
/// the damage by an attack, spell, or other cause.
#[derive(Debug)] // TODO: Manually implement Display.
pub struct Damage(pub(super) Vec<DamagePart>);

impl Display for Damage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.iter().enumerate().try_for_each(|(i, part)| {
            part.fmt(f)?;

            if i != (self.0.len() - 1) {
                write!(f, " + ")?;
            }

            Ok(())
        })
    }
}

impl From<DamagePart> for Damage {
    fn from(value: DamagePart) -> Self {
        Self(vec![value])
    }
}

impl Add for DamagePart {
    type Output = Damage;

    fn add(self, rhs: Self) -> Self::Output {
        Damage(vec![self, rhs])
    }
}

impl Add<Damage> for DamagePart {
    type Output = Damage;

    fn add(self, mut rhs: Damage) -> Self::Output {
        rhs.0.push(self);
        rhs
    }
}

impl Add<DamagePart> for Damage {
    type Output = Damage;

    fn add(mut self, rhs: DamagePart) -> Self::Output {
        self.0.push(rhs);
        self
    }
}

impl Add for Damage {
    type Output = Damage;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.0.append(&mut rhs.0);
        self
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add tests for:
    // * (Resistance, Vulnerability, Immunity) effects ( / 2, * 2, * 0 )
    // * (Resistance, Vulnerability) ordering
    // * (Resistance, Vulnerability, Immunity) idempotence (no stacking)
    use std::sync::Arc;

    use crate::core::{
        dice::DEvalTree,
        stats::{
            damage::{Resistance, Vulnerability},
            stat_block::StatBlock,
            AbilityScore,
        },
    };

    use super::{Bludgeoning, DamagePart, Fire};

    // #[test]
    // fn test_1() {
    //     fn s(raw: u8) -> AbilityScore {
    //         AbilityScore::new(raw).unwrap()
    //     }

    //     let mut rat = StatBlock::new(s(2), s(11), s(9), s(2), s(10), s(4), 3);

    //     // TODO: This is a nasty situation right here:
    //     //       there should be a way to do pre-init
    //     //       that should be memory safe.
    //     unsafe {
    //         let rat = Arc::get_mut_unchecked(&mut rat);
    //         rat.damages.add_effect(Fire, Vulnerability);
    //         rat.damages.add_effect(Fire, Resistance);
    //     };

    //     let dmg = DamagePart::environmental(Fire, DEvalTree::Roll(10))
    //         + DamagePart::environmental(Bludgeoning, DEvalTree::Roll(2));

    //     println!("Before: {dmg}");
    //     let dmg = rat.damages.calculate(dmg);

    //     println!("After: {dmg}");
    // }
}
