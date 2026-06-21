//! Tithe Drinker — `{W}{B}` 2/1 Creature — Vampire.
//! Lifelink. Extort (whenever you cast a spell, you may pay {W/B}; if you do,
//! each opponent loses 1 life and you gain that much life).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tithe Drinker");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);

    // GAP: keyword "Extort" — not in the modeled KeywordAbility surface (the
    // "whenever you cast a spell, you may pay {W/B}…" payload is not wired).
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
