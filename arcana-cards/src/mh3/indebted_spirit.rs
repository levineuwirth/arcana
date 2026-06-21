//! Indebted Spirit — `{W}` 1/1 Enchantment Creature — Spirit with
//! Afterlife 1.
//!
//! Rules text:
//! * Bestow {2}{W} — not a usable keyword variant; GAP'd.
//! * Afterlife 1.
//! * "Enchanted creature gets +1/+1 and has afterlife 1." — the bestow
//!   Aura static buff to the enchanted creature is not expressible as a
//!   triggered/activated ability; GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Indebted Spirit");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Afterlife(1)],
        ..Default::default()
    };

    // GAP: Bestow {2}{W} alternative-cost Aura mode.
    // GAP: static "Enchanted creature gets +1/+1 and has afterlife 1."
    reg.register(CardDefinition::new(name, chars))
}
