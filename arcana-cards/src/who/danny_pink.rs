//! Danny Pink — `{3}{U}` 4/3 Legendary Human Soldier Advisor (U) with Mentor.
//! The static "Creatures you control have '…counters…draw a card'" is a continuous
//! ability-granting static, not a triggered/activated ability on this card — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Danny Pink");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Mentor],
        ..Default::default()
    };

    // GAP: static "Creatures you control have 'Whenever one or more counters are
    // put on this creature for the first time each turn, draw a card.'" — granting
    // an arbitrary triggered ability to other permanents continuously is a
    // continuous static, not expressible as a triggered/activated ability here.

    reg.register(CardDefinition::new(name, chars))
}
