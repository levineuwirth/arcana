//! Harmonic Prodigy — `{1}{R}` 1/3 Human Wizard.
//!
//! Oracle text:
//! * Prowess — NOT in the supported `KeywordAbility` surface, so it cannot
//!   be listed. `keywords: vec![]`.
//! * "If a triggered ability of a Shaman or another Wizard you control
//!   triggers, that ability triggers an additional time." — a continuous
//!   static replacement on others' triggers; no `Effect` / static primitive
//!   models trigger-doubling.
//!
//! GAP: Prowess keyword not in supported surface (emit no keyword).
//! GAP: static "triggered ability triggers an additional time" — no
//! engine primitive for replacement-style trigger doubling.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Harmonic Prodigy");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
