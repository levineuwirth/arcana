//! Tithe Taker — `{1}{W}` 2/1 Human Soldier.
//! During your turn, spells your opponents cast cost {1} more and
//! abilities your opponents activate cost {1} more unless they're mana
//! abilities. (GAP'd — static cost-increase on opponents' spells/
//! abilities, not a triggered or activated ability.)
//! Afterlife 1.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tithe Taker");
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
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Afterlife(1)],
        // GAP: opponent spell/ability tax during your turn is a static
        // cost-modification effect.
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
