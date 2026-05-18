//! Nezumi Ronin — `{2}{B}` 3/1 Rat Samurai with Bushido 1.
//! Betrayers of Kamigawa common; an aggressive rat samurai that gets
//! +1/+1 whenever it blocks or becomes blocked.
//!
//! # Rules references
//!
//! * CR 702.44 — Bushido N. Whenever this creature blocks or becomes
//!   blocked, it gets +N/+N until end of turn. Engine wiring handles
//!   the triggered bonus; listing `Bushido(1)` in `keywords` is all
//!   that is required here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nezumi Ronin");
    let rat = reg.interner_mut().intern("Rat");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Bushido(1)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
