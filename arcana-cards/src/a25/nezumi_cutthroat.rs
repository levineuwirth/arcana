//! Nezumi Cutthroat — `{1}{B}` 2/1 Creature — Rat Warrior with Fear.
//! "This creature can't block."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nezumi Cutthroat");
    let rat = reg.interner_mut().intern("Rat");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Fear],
        ..Default::default()
    };

    // GAP: static "This creature can't block." — a pure continuous self-static
    // with no trigger or cost; no static-can't-block primitive in this card
    // class (ForbidBlocking is a one-shot targeted effect, not a self-static).
    reg.register(CardDefinition::new(name, chars))
}
