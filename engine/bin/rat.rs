#![feature(debug_closure_helpers)]

use xander_engine::core::{
    dice::{self, D20},
    stats::{skills::Persuasion, stat_block::StatBlock, AbilityScore},
};

fn main() {
    fn s(raw: u8) -> AbilityScore {
        AbilityScore::new(raw).unwrap()
    }

    dice::random_seed();

    let block = StatBlock::new(s(2), s(11), s(9), s(2), s(10), s(4));
    let persuasion = &block.skills[Persuasion];

    let persuasion_check = D20 + persuasion.get();
    println!(
        "Persuasion Check: {persuasion_check} => {}",
        persuasion_check.evaluate()
    );
}
