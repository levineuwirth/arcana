//! Nightshade Peddler — `{1}{G}` 1/1 Human Druid.
//! Soulbond.
//! As long as this creature is paired with another creature, both
//! creatures have deathtouch.
//!
//! Soulbond (pairing) is not a supported keyword and the engine has no
//! pairing model, so neither the keyword nor the paired-deathtouch
//! static is expressible. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nightshade Peddler");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: Soulbond keyword (pairing not modeled).
    // GAP: "while paired, both creatures have deathtouch" — pure static
    //      depending on the unmodeled pairing relationship.
    reg.register(CardDefinition::new(name, chars))
}
