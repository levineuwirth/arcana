//! Stern Mentor — `{3}{U}` 2/2 Human Wizard.
//! "Soulbond
//!  As long as this creature is paired with another creature, each of
//!  those creatures has '{T}: Target player mills two cards.'"
//!
//! GAP: Soulbond is not in the usable KeywordAbility surface (no pairing
//! mechanic), so `keywords: vec![]`.
//! GAP: the conditional granted activated ability ("as long as paired,
//! each has {T}: mill 2") depends on the unmodeled soulbond pairing and
//! grants an ability to another permanent — not expressible. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stern Mentor");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
