//! Daring Sleuth // Bearer of Overwhelming Truths — {1}{U} Creature — Human Rogue // Human Wizard (2/1)
//! Front: When you sacrifice a Clue, transform this creature.
//! Back: Prowess (GAP: Prowess not in engine keyword surface).
//!   Whenever this creature deals combat damage to a player, investigate.
//! GAP: "When you sacrifice a Clue" trigger condition not modeled
//!   (no ZoneChange filter for Clue sacrifice specifically).
//! GAP: Prowess keyword not in engine keyword surface.
//! GAP: Back-face-only triggered ability (investigate on combat damage) not modeled.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Daring Sleuth");
    let human_sub = reg.interner_mut().intern("Human");
    let rogue_sub = reg.interner_mut().intern("Rogue");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(human_sub);
    front_subtypes.0.insert(rogue_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Bearer of Overwhelming Truths");
    let human_sub2 = reg.interner_mut().intern("Human");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(human_sub2);
    back_subtypes.0.insert(wizard_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            // GAP: Prowess not in engine keyword surface.
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // GAP: "When you sacrifice a Clue, transform this creature" — Clue sacrifice
            // trigger condition not expressible. Using ZoneChange on Clue artifact as approximation
            // is not possible without subtype filtering on zone-change trigger. Omitting trigger.
            // GAP: Back-face combat damage -> investigate trigger not modeled.
    )
}
