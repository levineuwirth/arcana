//! Djinn Illuminatus — `{5}{U/R}{U/R}` 3/5 Djinn with Flying.
//!
//! Oracle:
//! * Flying
//! * Each instant and sorcery spell you cast has replicate. The replicate cost
//!   is equal to its mana cost.
//!
//! Flying is a base keyword. The replicate-granting line is a static ability
//! with no expressible primitive (there is no Replicate effect / cost shape),
//! so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Djinn Illuminatus");
    let djinn = reg.interner_mut().intern("Djinn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U/R}{U/R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static — "Each instant and sorcery spell you cast has replicate.
    // The replicate cost is equal to its mana cost." Granting replicate to
    // spells you cast is not expressible (no replicate cost/effect primitive).
    reg.register(CardDefinition::new(name, chars))
}
