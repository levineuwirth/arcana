//! Benalish Cavalry — `{1}{W}` 2/2 Human Knight with Flanking.
//! Tempest common; a white flanking knight that punishes blockers
//! without flanking by giving them -1/-1 until end of turn.
//!
//! # Rules references
//!
//! * CR 702.24 — Flanking. Whenever a creature without flanking
//!   blocks this creature, the blocking creature gets -1/-1 until
//!   end of turn. Engine wiring lives in the combat damage pipeline.
//!
//! Flanking is a fully-implemented keyword; listing it in `keywords`
//! is sufficient — the runtime pipelines do the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Benalish Cavalry");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flanking],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
