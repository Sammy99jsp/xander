extern crate xander_engine;
use xander_engine::core::stats::{AbilityModifier, AbilityScore};
use std::mem::{size_of, transmute};

#[rustfmt::skip]
#[allow(unused)]
enum TestAbilityScore {
    Valid(AbilityScore),
    P31, P32, P33, P34, P35, P36, P37, P38, P39, P40, P41, P42, P43, P44, P45, P46, P47, P48, P49, P50, P51, P52, P53, P54, P55, P56, P57, P58, P59, P60, P61, P62, P63, P64, P65, P66, P67, P68, P69, P70, P71, P72, P73, P74, P75, P76, P77, P78, P79, P80, P81, P82, P83, P84, P85, P86, P87, P88, P89, P90, P91, P92, P93, P94, P95, P96, P97, P98, P99, P100, P101, P102, P103, P104, P105, P106, P107, P108, P109, P110, P111, P112, P113, P114, P115, P116, P117, P118, P119, P120, P121, P122, P123, P124, P125, P126, P127, P128, P129, P130, P131, P132, P133, P134, P135, P136, P137, P138, P139, P140, P141, P142, P143, P144, P145, P146, P147, P148, P149, P150, P151, P152, P153, P154, P155, P156, P157, P158, P159, P160, P161, P162, P163, P164, P165, P166, P167, P168, P169, P170, P171, P172, P173, P174, P175, P176, P177, P178, P179, P180, P181, P182, P183, P184, P185, P186, P187, P188, P189, P190, P191, P192, P193, P194, P195, P196, P197, P198, P199, P200, P201, P202, P203, P204, P205, P206, P207, P208, P209, P210, P211, P212, P213, P214, P215, P216, P217, P218, P219, P220, P221, P222, P223, P224, P225, P226, P227, P228, P229, P230, P231, P232, P233, P234, P235, P236, P237, P238, P239, P240, P241, P242, P243, P244, P245, P246, P247, P248, P249, P250, P251, P252, P253, P254, P255,
    P0,
}

#[test]
fn test_niche_optimization_ability_score() {
    assert!(size_of::<TestAbilityScore>() == 1)
}

#[rustfmt::skip]
#[allow(unused)]
enum TestAbilityModifier {
    Valid(AbilityModifier),
    P11, P12, P13, P14, P15, P16, P17, P18, P19, P20, P21, P22, P23, P24, P25, P26, P27, P28, P29, P30, P31, P32, P33, P34, P35, P36, P37, P38, P39, P40, P41, P42, P43, P44, P45, P46, P47, P48, P49, P50, P51, P52, P53, P54, P55, P56, P57, P58, P59, P60, P61, P62, P63, P64, P65, P66, P67, P68, P69, P70, P71, P72, P73, P74, P75, P76, P77, P78, P79, P80, P81, P82, P83, P84, P85, P86, P87, P88, P89, P90, P91, P92, P93, P94, P95, P96, P97, P98, P99, P100, P101, P102, P103, P104, P105, P106, P107, P108, P109, P110, P111, P112, P113, P114, P115, P116, P117, P118, P119, P120, P121, P122, P123, P124, P125, P126, P127,
    N128, N127, N126, N125, N124, N123, N122, N121, N120, N119, N118, N117, N116, N115, N114, N113, N112, N111, N110, N109, N108, N107, N106, N105, N104, N103, N102, N101, N100, N99, N98, N97, N96, N95, N94, N93, N92, N91, N90, N89, N88, N87, N86, N85, N84, N83, N82, N81, N80, N79, N78, N77, N76, N75, N74, N73, N72, N71, N70, N69, N68, N67, N66, N65, N64, N63, N62, N61, N60, N59, N58, N57, N56, N55, N54, N53, N52, N51, N50, N49, N48, N47, N46, N45, N44, N43, N42, N41, N40, N39, N38, N37, N36, N35, N34, N33, N32, N31, N30, N29, N28, N27, N26, N25, N24, N23, N22, N21, N20, N19, N18, N17, N16, N15, N14, N13, N12, N11, N10, N9, N8, N7, N6,
}

#[test]
fn test_niche_optimization_ability_modifier() {
    assert!(size_of::<TestAbilityModifier>() == 1);
}

#[test]
fn test_ability_modifier_memory_layout() {
    // Assert that values -128 to -6 (inclusive) are not valid.
    (i8::MIN..=-6)
        .map(AbilityModifier::new)
        .for_each(|modifier| {
            assert!(modifier.is_none());
        });

    // Assert that valid values behave and have the some layout like regular i8's.
    (-5..=10)
        .map(|modifier| unsafe { transmute::<i8, Option<AbilityModifier>>(modifier) })
        .for_each(|modifier| {
            assert!(modifier.is_some());
        });

    // Assert that values 11 to 127 (inclusive) are not valid.
    (11..=i8::MAX)
        .map(AbilityModifier::new)
        .for_each(|modifier| {
            assert!(modifier.is_none());
        });
}

#[test]
fn test_ability_score_memory_layout() {
    // Assert that 0 is not a valid value.
    assert!(AbilityScore::new(0).is_none());

    // Assert that valid values have the same memory layout as i8's.
    (1..=30)
        .map(|score| unsafe { transmute::<i8, Option<AbilityScore>>(score) })
        .for_each(|score| {
            assert!(score.is_some());
        });

    // Assert that values from 31 to 255 (inclusive) are invalid.
    (31..=u8::MAX)
        .map(|score| unsafe { transmute::<u8, Option<AbilityScore>>(score) })
        .for_each(|score| {
            assert!(score.is_some());
        });
}
