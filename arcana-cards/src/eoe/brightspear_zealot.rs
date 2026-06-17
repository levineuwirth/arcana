//! Brightspear Zealot — `{2}{W}` 2/4 Human Soldier with Vigilance.
//! "Vigilance. This creature gets +2/+0 as long as you've cast two or more
//!  spells this turn."
//!
//! Vigilance is a base keyword. The conditional self-pump is a pure static
//! continuous effect (no trigger word, no cost) and is not expressible on the
//! MultiAbilityCreature surface.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brightspear Zealot");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: static "gets +2/+0 as long as you've cast two or more spells this turn" —
    // conditional continuous self-pump, no trigger/activation hook to express it.
    reg.register(CardDefinition::new(name, chars))
}
