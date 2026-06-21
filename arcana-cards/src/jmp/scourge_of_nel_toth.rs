//! Scourge of Nel Toth — `{5}{B}{B}` 6/6 Zombie Dragon with Flying.
//! "You may cast this creature from your graveyard by paying {B}{B} and
//! sacrificing two creatures rather than paying its mana cost."
//!
//! Flying is a base characteristic. The graveyard alternative-cost
//! casting permission is a static rules-altering ability with a
//! non-mana additional cost (sacrifice two creatures) that this card
//! class cannot express — GAP'd below.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scourge of Nel Toth");
    let zombie = reg.interner_mut().intern("Zombie");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "You may cast this from your graveyard by paying {B}{B} and
    // sacrificing two creatures rather than its mana cost." — static
    // alternative-cost permission with a non-mana additional cost; not
    // expressible in the MultiAbilityCreature surface.
    reg.register(CardDefinition::new(name, chars))
}
