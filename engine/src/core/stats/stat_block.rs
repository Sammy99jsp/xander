use std::{
    marker::PhantomData,
    ops::Index,
    rc::{Rc, Weak},
};

use crate::{
    core::{
        dice::DExpr,
        stats::{
            abilities::{
                Ability, Charisma as CHA, Constitution as CON, Dexterity as DEX,
                Intelligence as INT, Strength as STR, Wisdom as WIS,
            },
            skills::*,
            AbilityScore,
        },
    },
    utils::Proxy,
};

macro_rules! proxy_wrapper {
    ($id: ident <$gen: ident : $tr: path>, $inner: ty) => {
        #[repr(transparent)]
        pub struct $id<$gen>($inner, PhantomData<$gen>);

        impl<$gen: $tr> std::fmt::Debug for$id<$gen>  {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl<$gen: $tr> std::ops::Deref for $id<$gen> {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<$gen: $tr> std::ops::DerefMut for $id<$gen> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

#[derive(Debug)]
pub struct StatBlock {
    pub scores: AbilityScores,
    pub modifiers: AbilityModifiers,
    pub skills: Skills,
}

impl StatBlock {
    pub fn new(
        str: AbilityScore,
        dex: AbilityScore,
        con: AbilityScore,
        int: AbilityScore,
        wis: AbilityScore,
        cha: AbilityScore,
    ) -> Rc<Self> {
        Rc::new_cyclic(|this| Self {
            scores: AbilityScores::new(this, str, dex, con, int, wis, cha),
            modifiers: AbilityModifiers::new(this),
            skills: Skills::new(this),
        })
    }
}

proxy_wrapper!(AbilityScoreP<A: Ability>, Proxy<StatBlock, DExpr>);

#[allow(unused)]
#[repr(C)]
#[derive(Debug)]
pub struct AbilityScores {
    strength: AbilityScoreP<STR>,
    dexterity: AbilityScoreP<DEX>,
    constitution: AbilityScoreP<CON>,
    intelligence: AbilityScoreP<INT>,
    wisdom: AbilityScoreP<WIS>,
    charisma: AbilityScoreP<CHA>,
}

impl<A: Ability> AbilityScoreP<A> {
    pub fn new(ctx: Weak<StatBlock>, score: AbilityScore) -> Self {
        Self(Proxy::new(score.into(), ctx), PhantomData)
    }
}

impl AbilityScores {
    pub fn new(
        ctx: &Weak<StatBlock>,
        str: AbilityScore,
        dex: AbilityScore,
        con: AbilityScore,
        int: AbilityScore,
        wis: AbilityScore,
        cha: AbilityScore,
    ) -> Self {
        Self {
            strength: AbilityScoreP::new(ctx.clone(), str),
            dexterity: AbilityScoreP::new(ctx.clone(), dex),
            constitution: AbilityScoreP::new(ctx.clone(), con),
            intelligence: AbilityScoreP::new(ctx.clone(), int),
            wisdom: AbilityScoreP::new(ctx.clone(), wis),
            charisma: AbilityScoreP::new(ctx.clone(), cha),
        }
    }

    #[inline]
    pub fn get<A: Ability>(&self) -> &AbilityScoreP<A> {
        let idx = A::META.index();
        // SAFETY: Felids are aligned to 64 bytes, so our pointer arithmetic should be fine.
        let arr = self as *const Self as *const AbilityScoreP<()>;
        unsafe { arr.add(idx).cast::<AbilityScoreP<A>>().as_ref_unchecked() }
    }
}

impl<A: Ability> Index<A> for AbilityScores {
    type Output = AbilityScoreP<A>;

    #[inline]
    fn index(&self, _: A) -> &Self::Output {
        self.get::<A>()
    }
}

proxy_wrapper!(AbilityModifierP<A: Ability>, Proxy<StatBlock, DExpr>);

#[allow(unused)]
#[repr(C)]
#[derive(Debug)]
pub struct AbilityModifiers {
    strength: AbilityModifierP<STR>,
    dexterity: AbilityModifierP<DEX>,
    constitution: AbilityModifierP<CON>,
    intelligence: AbilityModifierP<INT>,
    wisdom: AbilityModifierP<WIS>,
    charisma: AbilityModifierP<CHA>,
}

impl<A: Ability> AbilityModifierP<A> {
    pub fn new(ctx: Weak<StatBlock>) -> Self {
        Self(
            Proxy::derived(
                |stats| {
                    let score =
                        AbilityScore::new(stats.scores.get::<A>().get().result() as u8).unwrap();
                    score.modifier().into()
                },
                ctx,
            ),
            PhantomData,
        )
    }
}

impl AbilityModifiers {
    // TODO: Replace maybe with a proc macro.
    pub fn new(ctx: &Weak<StatBlock>) -> Self {
        Self {
            strength: AbilityModifierP::new(ctx.clone()),
            dexterity: AbilityModifierP::new(ctx.clone()),
            constitution: AbilityModifierP::new(ctx.clone()),
            intelligence: AbilityModifierP::new(ctx.clone()),
            wisdom: AbilityModifierP::new(ctx.clone()),
            charisma: AbilityModifierP::new(ctx.clone()),
        }
    }

    #[inline]
    pub fn get<A: Ability>(&self) -> &AbilityModifierP<A> {
        let idx = A::META.index();
        // SAFETY: Felids are aligned to 64 bytes, so our pointer arithmetic should be fine.
        let arr = self as *const Self as *const AbilityModifierP<()>;
        unsafe {
            arr.add(idx)
                .cast::<AbilityModifierP<A>>()
                .as_ref_unchecked()
        }
    }
}

impl<A: Ability> Index<A> for AbilityModifiers {
    type Output = AbilityModifierP<A>;

    #[inline]
    fn index(&self, _: A) -> &Self::Output {
        self.get::<A>()
    }
}

proxy_wrapper!(SkillP<S: Skill>, Proxy<StatBlock, DExpr>);
#[allow(unused)]
#[derive(Debug)]
#[repr(C)]
pub struct Skills {
    athletics: SkillP<Athletics>,
    acrobatics: SkillP<Acrobatics>,
    sleight_of_hand: SkillP<SleightOfHand>,
    stealth: SkillP<Stealth>,
    arcana: SkillP<Arcana>,
    history: SkillP<History>,
    investigation: SkillP<Investigation>,
    nature: SkillP<Nature>,
    religion: SkillP<Religion>,
    animal_handling: SkillP<AnimalHandling>,
    insight: SkillP<Insight>,
    medicine: SkillP<Medicine>,
    perception: SkillP<Perception>,
    survival: SkillP<Survival>,
    deception: SkillP<Deception>,
    intimidation: SkillP<Intimidation>,
    performance: SkillP<Performance>,
    persuasion: SkillP<Persuasion>,
}

impl Skills {
    pub fn new(ctx: &Weak<StatBlock>) -> Self {
        Self {
            athletics: SkillP::new(ctx.clone()),
            acrobatics: SkillP::new(ctx.clone()),
            sleight_of_hand: SkillP::new(ctx.clone()),
            stealth: SkillP::new(ctx.clone()),
            arcana: SkillP::new(ctx.clone()),
            history: SkillP::new(ctx.clone()),
            investigation: SkillP::new(ctx.clone()),
            nature: SkillP::new(ctx.clone()),
            religion: SkillP::new(ctx.clone()),
            animal_handling: SkillP::new(ctx.clone()),
            insight: SkillP::new(ctx.clone()),
            medicine: SkillP::new(ctx.clone()),
            perception: SkillP::new(ctx.clone()),
            survival: SkillP::new(ctx.clone()),
            deception: SkillP::new(ctx.clone()),
            intimidation: SkillP::new(ctx.clone()),
            performance: SkillP::new(ctx.clone()),
            persuasion: SkillP::new(ctx.clone()),
        }
    }

    #[inline]
    pub fn get<S: Skill>(&self) -> &SkillP<S> {
        let idx = S::META.index();
        // SAFETY: Felids are aligned to 64 bytes, so our pointer arithmetic should be fine.
        let arr = self as *const Self as *const AbilityModifierP<()>;
        unsafe { arr.add(idx).cast::<SkillP<S>>().as_ref_unchecked() }
    }
}

impl<S: Skill> Index<S> for Skills {
    type Output = SkillP<S>;

    #[inline]
    fn index(&self, _: S) -> &Self::Output {
        self.get::<S>()
    }
}

impl<S: Skill> SkillP<S> {
    pub fn new(ctx: Weak<StatBlock>) -> Self {
        Self(
            Proxy::derived(|ctx| ctx.modifiers.get::<S::Base>().get(), ctx),
            PhantomData,
        )
    }
}
