//! Timber Paladin — `{1}{G}` 1/1 Artifact Creature — Knight.
//!
//! All three lines are pure static continuous abilities (conditional
//! base-P/T-setting / keyword grants based on the number of Auras
//! enchanting this creature — no trigger word, no activation cost), so
//! none can be expressed as a triggered/activated ability. Bones only.
//!
//! * GAP (static): "As long as this creature is enchanted by exactly
//!   one Aura, it has base power and toughness 3/3."
//! * GAP (static): "As long as this creature is enchanted by exactly
//!   two Auras, it has base power and toughness 5/5 and vigilance."
//! * GAP (static): "As long as this creature is enchanted by three or
//!   more Auras, it has base power and toughness 10/10, vigilance, and
//!   trample."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Timber Paladin");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
