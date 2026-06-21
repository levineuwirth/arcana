//! Guard Gomazoa — `{2}{U}` 1/3 Creature — Jellyfish.
//! Defender, flying.
//! Prevent all combat damage that would be dealt to this creature.
//! (Static replacement — GAP'd: no Effect expresses a self-scoped,
//! combat-only prevention shield as a continuous static ability.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Guard Gomazoa");
    let jellyfish = reg.interner_mut().intern("Jellyfish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jellyfish);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Defender, KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static replacement "Prevent all combat damage that would be
    // dealt to this creature" — not expressible as a continuous static.
    reg.register(CardDefinition::new(name, chars))
}
