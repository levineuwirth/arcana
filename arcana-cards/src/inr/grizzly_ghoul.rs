//! Grizzly Ghoul — `{2}{B}{G}` 4/3 Zombie Bear (B/G). Trample. Enters with
//! a +1/+1 counter on it for each creature that died this turn (static
//! enters-with replacement → GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grizzly Ghoul");
    let zombie = reg.interner_mut().intern("Zombie");
    let bear = reg.interner_mut().intern("Bear");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(bear);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: static "enters with a +1/+1 counter on it for each creature
    // that died this turn" — an enters-with replacement, not a
    // trigger/activated ability.

    reg.register(CardDefinition::new(name, chars))
}
