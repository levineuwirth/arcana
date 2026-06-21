//! Zuo Ci, the Mocking Sage — `{1}{G}{G}` 1/2 Legendary Human Advisor.
//!
//! Oracle:
//! * Hexproof.
//! * "Zuo Ci can't be blocked by creatures with horsemanship." — a
//!   keyword-filtered block restriction; no expressible primitive (the
//!   evasion form has no "can't be blocked by creatures with <keyword>"
//!   variant), GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zuo Ci, the Mocking Sage");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };

    // GAP: "can't be blocked by creatures with horsemanship" — a
    // keyword-filtered block restriction with no expressible primitive.
    reg.register(CardDefinition::new(name, chars))
}
