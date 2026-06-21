//! Ardent Soldier — `{1}{W}` 1/2 Human Soldier with Vigilance.
//! Kicker {2}. (GAP: Kicker is not an available KeywordAbility.)
//! If this creature was kicked, it enters with a +1/+1 counter on it. (GAP:
//! depends on the unmodeled kicker state.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ardent Soldier");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        // GAP: keyword — "Kicker {2}" is not an available KeywordAbility.
        ..Default::default()
    };

    // GAP: trigger — "If this creature was kicked, it enters with a +1/+1
    // counter on it" depends on the unmodeled kicker-was-paid state.

    reg.register(CardDefinition::new(name, chars))
}
