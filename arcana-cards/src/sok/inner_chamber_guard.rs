//! Inner-Chamber Guard — `{1}{W}` 0/2 Human Samurai with Bushido 2.
//! Saviors of Kamigawa common; a defensive samurai that surges to
//! +2/+2 whenever it blocks or becomes blocked.
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
    let name = reg.interner_mut().intern("Inner-Chamber Guard");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Bushido(2)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
