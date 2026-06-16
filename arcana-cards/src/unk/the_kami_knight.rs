//! The Kami Knight — `{2}{W}` 3/3 Legendary Creature — Spirit Warrior (W).
//!
//! Both printed abilities are pure static continuous abilities with no
//! trigger word and no cost, and neither is expressible with the demonstrated
//! API, so both are GAP'd. Goad is not an available `KeywordAbility` variant
//! (it appears in the Scryfall keyword list only because of the second static
//! line), so the keyword vec is empty.
//!
//! * GAP: static — "Other creatures you control gain all bonuses conferred by
//!   equipment attached to this creature."
//! * GAP: static — "Creatures you control can't be goaded."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Kami Knight");
    let spirit = reg.interner_mut().intern("Spirit");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
