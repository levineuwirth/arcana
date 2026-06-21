//! Sapphire Leech — `{1}{U}` 2/2 Leech with Flying.
//! "Flying
//!  Blue spells you cast cost {U} more to cast."
//!
//! Flying is a keyword. "Blue spells you cast cost {U} more to cast" is a
//! cost-increase static with no demonstrated hook on this shape — GAP.

// GAP (static): "Blue spells you cast cost {U} more to cast" — no
// cost-modification static hook.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sapphire Leech");
    let leech = reg.interner_mut().intern("Leech");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(leech);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
