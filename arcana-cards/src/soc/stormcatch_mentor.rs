//! Stormcatch Mentor — `{U}{R}` 1/1 Otter Wizard with Haste.
//! Prowess (GAP: not in supported keyword surface).
//! Instant and sorcery spells you cast cost {1} less to cast (GAP: static
//! cost reduction not expressible).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stormcatch Mentor");
    let otter = reg.interner_mut().intern("Otter");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(otter);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Prowess is not in the supported KeywordAbility surface.
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };
    // GAP: static "Instant and sorcery spells you cast cost {1} less to cast"
    // (cost reduction) is not expressible here.
    reg.register(CardDefinition::new(name, chars))
}
