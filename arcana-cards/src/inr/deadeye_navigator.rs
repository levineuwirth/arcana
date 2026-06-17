//! Deadeye Navigator — `{4}{U}{U}` 5/5 Spirit.
//! Soulbond; "As long as Deadeye Navigator is paired with another creature,
//!  each of those creatures has '{1}{U}: Exile this creature, then return it
//!  to the battlefield under your control.'"
//!
//! Soulbond is not an available KeywordAbility variant, and the paired-grant
//! is a static conditional ability-grant. Neither is expressible. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deadeye Navigator");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: Soulbond is not an available KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: the paired-while static granting "{1}{U}: Exile this creature, then
    // return it to the battlefield under your control" to both paired creatures
    // is a conditional static ability-grant; not expressible.
    reg.register(CardDefinition::new(name, chars))
}
