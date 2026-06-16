//! War-Wing Siren — `{2}{U}` 1/3 Siren Soldier with Flying.
//! Heroic — "Whenever you cast a spell that targets this creature, put a
//! +1/+1 counter on this creature." is GAP: there is no TriggerCondition
//! for "a spell you cast that targets this creature" (Heroic).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("War-Wing Siren");
    let siren = reg.interner_mut().intern("Siren");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siren);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: Heroic — no TriggerCondition matches "whenever you cast a
    // spell that targets this creature".
    reg.register(CardDefinition::new(name, chars))
}
