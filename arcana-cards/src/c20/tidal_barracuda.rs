//! Tidal Barracuda — `{3}{U}` 3/4 Fish (blue).
//! "Any player may cast spells as though they had flash.
//!  Your opponents can't cast spells during your turn."
//!
//! Both lines are static rule-altering abilities (timing-permission grants
//! and a casting restriction). Neither is expressible with the demonstrated
//! triggered/activated API, so both are GAP'd and only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tidal Barracuda");
    let fish = reg.interner_mut().intern("Fish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "Any player may cast spells as though they had flash." — static
    // timing-permission grant, not expressible.
    // GAP: "Your opponents can't cast spells during your turn." — static
    // casting restriction, not expressible.
    reg.register(CardDefinition::new(name, chars))
}
