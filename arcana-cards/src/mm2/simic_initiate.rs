//! Simic Initiate — `{G}` 0/0 Human Mutant with Graft 1.
//!
//! # Rules references
//!
//! * CR 702.57 — Graft. This creature enters with a +1/+1 counter on it.
//!   Whenever another creature enters, you may move a +1/+1 counter from
//!   this creature onto it.
//!
//! Graft is not in the demonstrated KeywordAbility API; this file is a
//! best-effort stub. The verify pipeline will flag the gap.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Simic Initiate");
    let human = reg.interner_mut().intern("Human");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
