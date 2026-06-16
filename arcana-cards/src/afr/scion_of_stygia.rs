//! Scion of Stygia — `{2}{U}` 2/1 Tiefling Shaman with Flash.
//! "Cone of Cold — When this creature enters, choose target creature an
//! opponent controls, then roll a d20. 1–9: Tap that creature. 10–20: Tap
//! that creature. It doesn't untap during its controller's next untap step."
//!
//! Flash is a base characteristic. The d20-roll ETB is not expressible: there
//! is no dice-roll primitive, so the branch on the roll result cannot be
//! modeled. GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scion of Stygia");
    let tiefling = reg.interner_mut().intern("Tiefling");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tiefling);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    // GAP: "Cone of Cold" ETB — rolls a d20 and branches on the result; no
    // dice-roll primitive exists to express the d20 / result split.
    reg.register(CardDefinition::new(name, chars))
}
