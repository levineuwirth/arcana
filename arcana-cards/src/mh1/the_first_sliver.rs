//! The First Sliver — `{W}{U}{B}{R}{G}` 7/7 Legendary Sliver with Cascade.
//! "Sliver spells you cast have cascade."
//!
//! Cascade is a base keyword (the cast-time exile-and-free-cast is
//! engine-wired). The static "Sliver spells you cast have cascade" is
//! a continuous ability granting cascade to other spells you cast —
//! not a triggered/activated ability and not expressible here, so GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The First Sliver");
    let sliver = reg.interner_mut().intern("Sliver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sliver);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Cascade],
        ..Default::default()
    };

    // GAP: static "Sliver spells you cast have cascade" — a continuous
    // ability granting cascade to other spells; not expressible as a
    // triggered/activated ability on this card class.
    reg.register(CardDefinition::new(name, chars))
}
