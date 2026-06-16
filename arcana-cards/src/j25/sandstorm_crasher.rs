//! Sandstorm Crasher — `{3}{R}` 3/4 Minotaur Berserker Wizard with Trample.
//! "You may exert this creature as it attacks. When you do, create a tapped and
//! attacking token that's a copy of target creature you control. Sacrifice the
//! token at the beginning of the next end step."
//!
//! Trample is a base keyword. Exert is not in the usable keyword surface, and
//! the exert-gated "create a tapped and attacking token copy, then sacrifice
//! it" payload has no demonstrated API surface (no exert trigger gate, no
//! tapped-and-attacking token-copy effect) — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sandstorm Crasher");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let berserker = reg.interner_mut().intern("Berserker");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);
    subtypes.0.insert(berserker);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: exert-gated "create a tapped and attacking token that's a copy of
    //      target creature you control, sacrifice at next end step" — no exert
    //      trigger gate nor tapped-and-attacking token-copy effect available.
    reg.register(CardDefinition::new(name, chars))
}
