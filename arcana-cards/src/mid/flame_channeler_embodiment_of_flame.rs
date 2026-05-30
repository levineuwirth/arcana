//! Flame Channeler // Embodiment of Flame — `{1}{R}` Creature — Human Wizard
//! 2/2 (front), transforms to Creature — Elemental Wizard (back).
//!
//! Front: When a spell you control deals damage, transform this creature.
//! Back: Whenever a spell you control deals damage, put a flame counter on
//!       this creature. {1}, Remove a flame counter from this creature: Exile
//!       the top card of your library. You may play that card this turn.
//!
//! # GAP notes
//! - "When a spell you control deals damage" — there is no
//!   TriggerCondition variant for SpellDealsDamage in the demonstrated API.
//!   // GAP: TriggerCondition::SpellDealsDamage not modeled; front-face
//!   // transform trigger omitted.
//! - Back-face flame counter trigger and activated ability ({1}, remove a
//!   flame counter: exile top card, may play this turn) are back-face-only
//!   abilities not auto-installed on transform.
//!   // GAP: back-face-only triggered ability not modeled.
//!   // GAP: back-face activated ability ({1}, remove flame counter: impulse)
//!   // not modeled.
//! - "Transform" Scryfall keyword is a marker; not a KeywordAbility variant.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flame Channeler");
    let human_sub = reg.interner_mut().intern("Human");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(human_sub);
    front_subtypes.0.insert(wizard_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Embodiment of Flame");
    let elemental_sub = reg.interner_mut().intern("Elemental");
    let wizard_sub2 = reg.interner_mut().intern("Wizard");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(elemental_sub);
    back_subtypes.0.insert(wizard_sub2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: front-face trigger "when a spell you control deals damage, transform"
    // not modeled — TriggerCondition::SpellDealsDamage not in the API.
    // GAP: back-face-only triggered ability not modeled.
    // GAP: back-face activated ability not modeled.
    reg.register(CardDefinition::new(name, chars).with_transform_back(back))
}
