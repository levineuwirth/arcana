//! Maelstrom Wanderer — `{5}{G}{U}{R}` Legendary 7/5 Elemental.
//! "Creatures you control have haste." (static)
//! "Cascade, cascade" — two cascade triggers when this spell is cast.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Maelstrom Wanderer");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{U}{R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: static — "Creatures you control have haste" is a continuous
    // ability, not a triggered/activated ability.
    // GAP: "Cascade, cascade" fires on THIS spell's own cast; no self-cast
    // TriggerCondition is available to host the Effect::Cascade body for this
    // card (matches the ethersworn_sphinx precedent).
    reg.register(CardDefinition::new(name, chars))
}
