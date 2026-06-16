//! Tajic, Blade of the Legion — `{2}{R}{W}` 2/2 Legendary Human
//! Soldier with Indestructible.
//! Battalion — Whenever Tajic and at least two other creatures attack,
//! Tajic gets +5/+5 until end of turn.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tajic, Blade of the Legion");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Indestructible],
        ..Default::default()
    };

    // GAP: Battalion — "Whenever Tajic and at least two other creatures
    // attack" has no matching TriggerCondition variant (SelfAttacks
    // lacks the "two other attackers" gate, and no such intervening-if
    // helper exists); firing it unconditionally would be a wrong card.
    reg.register(CardDefinition::new(name, chars))
}
