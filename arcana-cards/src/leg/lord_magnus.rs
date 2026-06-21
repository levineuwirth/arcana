//! Lord Magnus — `{3}{G}{W}{W}` 4/3 Legendary Human Druid.
//! First strike.
//! "Creatures with plainswalk can be blocked as though they didn't have
//!  plainswalk."
//! "Creatures with forestwalk can be blocked as though they didn't have
//!  forestwalk."
//!
//! First strike is wired. The two landwalk-negation statics have no expressible
//! effect (no "ignore landwalk for blocking" primitive) — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lord Magnus");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{W}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // GAP: static — "creatures with plainswalk/forestwalk can be blocked as
    // though they didn't have it." No expressible landwalk-negation effect.
    reg.register(CardDefinition::new(name, chars))
}
