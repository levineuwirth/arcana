//! Jade Avenger — `{1}{G}` 2/2 Frog Samurai with Bushido 2.
//! Kamigawa: Neon Dynasty common; gets +2/+2 whenever it blocks or
//! becomes blocked.
//!
//! # Rules references
//!
//! * CR 702.44 — Bushido N. Whenever this creature blocks or becomes
//!   blocked, it gets +N/+N until end of turn. Engine wiring handles
//!   the triggered bonus; listing `Bushido(2)` in `keywords` is all
//!   that is required here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jade Avenger");
    let frog = reg.interner_mut().intern("Frog");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);
    subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Bushido(2)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
