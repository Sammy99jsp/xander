use std::rc::Rc;

use xander::{
    core::{
        dice::{self, DExpr},
        stats::{
            abilities::{Dexterity, Strength},
            check::{Check, Save, DC},
            skills::{self, Persuasion},
            stat_block::{Proficiency, SkillP, StatBlock},
            AbilityScore,
        },
    },
    utils::{proxy::Dispatch, ProxyPart},
};

fn main() {
    // fn s(raw: u8) -> AbilityScore {
    //     AbilityScore::new(raw).unwrap()
    // }

    // dice::random_seed();

    // let rat_stat_block = StatBlock::new(s(2), s(11), s(9), s(2), s(10), s(4), 3);

    // let mut proxy: SkillP<skills::Acrobatics> = SkillP::new(Rc::downgrade(&rat_stat_block));
    // proxy.insert(Proficiency);

    // let res = rat_stat_block.save(Save::new(DC::HARD, Dexterity));
    // println!("{res:?}")
}
