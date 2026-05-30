//! Reckless Waif // Merciless Predator — `{R}` Human Rogue Werewolf creature 1/1.
//! At the beginning of each upkeep, if no spells were cast last turn, transform.
//! Back face "Merciless Predator": Creature — Werewolf (no P/T given; see oracle).
//! At the beginning of each upkeep, if a player cast two or more spells last turn, transform.
//!
//! GAP: The werewolf transform trigger conditions ("no spells cast last turn" /
//! "a player cast two or more spells last turn") require tracking spells cast
//! across turns, which is not available in the engine's per-turn counters
//! (those reset on the NEW turn, not the LAST turn). The exact werewolf day/night
//! condition is engine debt. Both transform triggers are omitted.
//! GAP: back-face-only triggered ability (back-to-front transform) not modeled.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reckless Waif");
    let human_sub = reg.interner_mut().intern("Human");
    let rogue_sub = reg.interner_mut().intern("Rogue");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(rogue_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    // Back face "Merciless Predator": Creature — Werewolf 3/2
    let back_name = reg.interner_mut().intern("Merciless Predator");
    let mut back_subtypes = SubtypeSet::default();
    let werewolf_sub2 = reg.interner_mut().intern("Werewolf");
    back_subtypes.0.insert(werewolf_sub2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: Werewolf transform triggers (front→back: "no spells cast last turn",
    // back→front: "two or more spells cast last turn") require cross-turn spell
    // tracking which is engine debt — neither trigger is modeled.
    reg.register(CardDefinition::new(name, chars).with_transform_back(back))
}
