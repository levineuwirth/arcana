//! A-Gutter Skulker // A-Gutter Shortcut
//!
//! Front (Creature — Spirit 3/3, {2}{U}):
//!   Gutter Skulker can't be blocked as long as it's attacking alone.
//!   Disturb {2}{U} (cast from graveyard transformed).
//! Back (Enchantment — Aura):
//!   Enchant creature. Enchanted creature gets +3/+0 and can't be blocked while attacking alone.
//!   If Gutter Shortcut would be put into a graveyard, exile it instead.
//!
//! GAP: "can't be blocked as long as it's attacking alone" — static conditional evasion not
//!       expressible (CantBeBlocked only fires as an effect, not a static layer). Omitted.
//! GAP: Disturb cast mechanic not modeled (graveyard-as-alt-cast-zone is engine debt).
//! GAP: back-face Aura +3/+0 pump and conditional-block are back-face static layers; not modeled.
//! GAP: "if would be put into a graveyard, exile instead" replacement is engine debt.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Gutter Skulker");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("A-Gutter Shortcut");
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

    reg.register(CardDefinition::new(name, chars).with_transform_back(back))
}
