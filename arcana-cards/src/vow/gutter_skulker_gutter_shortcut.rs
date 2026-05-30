//! Gutter Skulker // Gutter Shortcut — `{3}{U}` Spirit creature 3/3.
//! Front face: This creature can't be blocked as long as it's attacking alone.
//! Disturb {3}{U} — cast from graveyard transformed.
//! Back face (Aura Enchantment): Enchant creature; enchanted creature can't be
//! blocked as long as it's attacking alone. If Gutter Shortcut would be put into
//! a graveyard from anywhere, exile it instead.
//!
//! GAP: Disturb (cast from graveyard as transformed) is not modeled — no
//!   graveyard-cast mechanic in engine. The card is registered as a normal
//!   transform creature.
//! GAP: "Can't be blocked as long as it's attacking alone" — the conditional
//!   is not expressible; Effect::CantBeBlocked with WhileSourceOnBattlefield
//!   approximates it as an unconditional static evasion.
//! GAP: Back face's "exile instead of graveyard" replacement effect not modeled.
//! GAP: Back face is an Aura — Enchant targeting and attachment not modeled.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gutter Skulker");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Gutter Shortcut");
    let aura_sub = reg.interner_mut().intern("Aura");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(aura_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: "Can't be blocked as long as it's attacking alone" is a conditional
    // evasion. No conditional CantBeBlocked trigger available; the static effect
    // is deferred.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
