//! Melek, Reforged Researcher — `{3}{U}{R}` */* Legendary Weird
//! Detective.
//!
//! Melek's power and toughness are each equal to twice the number of
//! instant and sorcery cards in your graveyard.  (CDA static — GAP)
//! The first instant or sorcery spell you cast each turn costs {3} less
//! to cast.  (cost-reduction static — GAP)
//!
//! Both abilities are pure statics with no triggered/activated surface.
//! Bones are recorded; the `*/*` P/T is `PtValue::Star`. Neither static
//! is expressible with the demonstrated triggered/activated API.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Melek, Reforged Researcher");
    let weird = reg.interner_mut().intern("Weird");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(weird);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    // GAP: "power and toughness equal to twice the instant/sorcery cards
    // in your graveyard" — characteristic-defining static, no
    // triggered/activated surface.
    // GAP: "first instant or sorcery spell you cast each turn costs {3}
    // less" — cost-reduction static, not expressible here.
    reg.register(CardDefinition::new(name, chars))
}
