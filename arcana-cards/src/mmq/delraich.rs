//! Delraich — `{6}{B}` 6/6 Creature — Horror.
//! Trample.
//! "You may sacrifice three black creatures rather than pay this spell's mana
//! cost." — an alternative-cost (sacrifice three black creatures) cast option;
//! alternative casting costs are not in the demonstrated keyword / activation
//! surface, so this is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Delraich");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: alternative cost "You may sacrifice three black creatures rather than
    // pay this spell's mana cost" — alternative casting costs are not expressible.
    reg.register(CardDefinition::new(name, chars))
}
