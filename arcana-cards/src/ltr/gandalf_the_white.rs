//! Gandalf the White — `{3}{W}{W}` Legendary 4/5 Avatar Wizard with Flash.
//!
//! GAP (static): "You may cast legendary spells and artifact spells as though
//! they had flash" is a casting-permission static — not expressible.
//! GAP (static): "If a legendary permanent or an artifact entering or leaving
//! the battlefield causes a triggered ability ... to trigger, that ability
//! triggers an additional time" is a trigger-doubling replacement — not
//! expressible. Both are pure statics, so only the Flash bones are emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gandalf the White");
    let avatar = reg.interner_mut().intern("Avatar");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
