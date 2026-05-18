//! Hired Blade — `{2}{B}` 3/2 Human Assassin with Flash. A black creature
//! from Core Set 2019; can be cast at instant speed to ambush attackers
//! or interact at the end of an opponent's turn.
//!
//! # Rules references
//!
//! * CR 702.8 — Flash. You may cast this spell any time you could cast an
//!   instant. Engine wiring permits casting during any step where instants
//!   are legal.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hired Blade");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
