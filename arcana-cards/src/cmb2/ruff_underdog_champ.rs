//! Ruff, Underdog Champ — `{2}{W}{W}` 3/2 Legendary Dog Soldier with First
//! strike and Lifelink.
//! "All Hounds are Dogs." (static type-changing rule — GAP)
//! "First strike, lifelink"
//! "Underdog — If you've lost a game this match, Ruff, Underdog Champ and
//!  other Dogs you control get +1/+1." (GAP)
//!
//! GAP: "All Hounds are Dogs" is a static type-changing rule with no trigger
//!      or cost — not expressible.
//! GAP: the Underdog anthem is a pure static, conditioned on match history
//!      ("if you've lost a game this match"); neither the static nor the
//!      match-loss condition is expressible.
//! "Underdog" is an ability word, not a usable `KeywordAbility`, so only
//! First strike and Lifelink go in the keyword line.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ruff, Underdog Champ");
    let dog = reg.interner_mut().intern("Dog");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
