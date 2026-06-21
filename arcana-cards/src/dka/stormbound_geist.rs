//! Stormbound Geist — `{1}{U}{U}` 2/2 blue Spirit.
//! Flying.
//! "This creature can block only creatures with flying." (static block
//! restriction — no expressible primitive, GAP'd.)
//! Undying (When this creature dies, if it had no +1/+1 counters on it,
//! return it to the battlefield with a +1/+1 counter on it.)
//!
//! Flying and Undying are base keyword characteristics; the runtime
//! pipelines wire both. The "can block only flying" combat restriction
//! has no demonstrated primitive, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stormbound Geist");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Undying],
        ..Default::default()
    };

    // GAP (static): "This creature can block only creatures with flying."
    // No block-restriction primitive in the demonstrated API.

    reg.register(CardDefinition::new(name, chars))
}
