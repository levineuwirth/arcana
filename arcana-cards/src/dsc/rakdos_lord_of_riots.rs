//! Rakdos, Lord of Riots — `{B}{B}{R}{R}` 6/6 Legendary Creature — Demon.
//!
//! Flying, trample
//! * You can't cast Rakdos unless an opponent lost life this turn. (Casting
//!   restriction — not expressible; GAP'd.)
//! * Creature spells you cast cost {1} less to cast for each 1 life your
//!   opponents have lost this turn. (Cost-reduction static — not
//!   expressible; GAP'd.)
//!
//! GAP: both non-keyword lines are casting-time statics with no expressible
//! primitive (a cast restriction and a cost reduction).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rakdos, Lord of Riots");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}{R}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
