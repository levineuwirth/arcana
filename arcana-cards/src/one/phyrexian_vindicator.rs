//! Phyrexian Vindicator — `{W}{W}{W}{W}` 5/5 Phyrexian Horror with Flying.
//! "If damage would be dealt to this creature, prevent that damage. When damage
//! is prevented this way, this creature deals that much damage to any other target."
//!
//! GAP: the damage-prevention replacement effect ("if damage would be dealt to
//! this creature, prevent that damage") is a static replacement, not a
//! triggered/activated ability, and its linked "when damage is prevented this
//! way" trigger has no matching `TriggerCondition` variant. Both are omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian Vindicator");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
