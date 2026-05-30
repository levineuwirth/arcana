//! Hookhand Mariner // Riphook Raider — `{3}{G}` green Creature — Human Werewolf 4/4.
//!
//! Front: Daybound (GAP: day/night cycle not modeled — transform condition not expressible).
//!
//! Back: Riphook Raider — Creature — Werewolf.
//! This creature can't be blocked by creatures with power 2 or less.
//! Nightbound (GAP: day/night cycle not modeled).
//!
//! GAP: Daybound/Nightbound werewolf transform triggers not modeled.
//! GAP: back-face-only "can't be blocked by creatures with power 2 or less" not modeled
//!      (a back-face-only triggered/static ability that is not auto-installed on transform).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hookhand Mariner");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        // GAP: Daybound not in keyword surface
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Riphook Raider");
    let werewolf_back = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_back);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![],
            // GAP: Nightbound not in keyword surface
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
        // GAP: Daybound/Nightbound transform triggers (day/night cycle) not modeled.
        // GAP: back-face "can't be blocked by creatures with power 2 or less"
        //      is a back-face-only static ability not auto-installed on transform.
    )
}
