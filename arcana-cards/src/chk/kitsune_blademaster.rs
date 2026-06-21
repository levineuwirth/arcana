//! Kitsune Blademaster — `{2}{W}` 2/2 white Fox Samurai.
//!
//! Oracle:
//! * First strike — base keyword.
//! * Bushido 1 (Whenever this creature blocks or becomes blocked, it gets
//!   +1/+1 until end of turn.) — parametrized keyword `Bushido(1)`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kitsune Blademaster");
    let fox = reg.interner_mut().intern("Fox");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fox);
    subtypes.0.insert(samurai);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Bushido(1)],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
