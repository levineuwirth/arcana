//! Voidstone Gargoyle — `{3}{W}{W}` 3/3 Creature — Gargoyle.
//! Flying.
//! "As this creature enters, choose a nonland card name."
//! "Spells with the chosen name can't be cast."
//! "Activated abilities of sources with the chosen name can't be activated."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Voidstone Gargoyle");
    let gargoyle = reg.interner_mut().intern("Gargoyle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gargoyle);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    // GAP: "As this creature enters, choose a nonland card name" + the two
    // resulting "can't be cast" / "can't be activated" name-locking static
    // replacement effects are not expressible with the demonstrated
    // triggered/activated/keyword API (no chosen-name continuous restriction
    // effect). Only the bones + Flying are emitted.
    reg.register(CardDefinition::new(name, chars))
}
