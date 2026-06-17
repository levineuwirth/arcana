//! Haunting Figment — `{1}{U}` 2/1 Illusion.
//! Vigilance.
//! "This creature can't be blocked as long as you've cast an instant or sorcery
//! spell this turn." (conditional static evasion — GAP)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Haunting Figment");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: "can't be blocked while you've cast an instant or sorcery this turn"
    //      is a conditional static evasion ability, not a trigger/activation.
    reg.register(CardDefinition::new(name, chars))
}
