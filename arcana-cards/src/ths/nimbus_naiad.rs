//! Nimbus Naiad — `{2}{U}` 2/2 Enchantment Creature — Nymph.
//!
//! * Flying.
//! * Bestow {4}{U} — `Bestow` is not in the supported KeywordAbility
//!   surface for this card class; GAP'd.
//! * "Enchanted creature gets +2/+2 and has flying." — the Aura-mode
//!   static (active only when cast for its bestow cost and attached) has
//!   no expressible primitive here; GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nimbus Naiad");
    let nymph = reg.interner_mut().intern("Nymph");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nymph);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: Bestow {4}{U} (keyword not supported) and the
        // "Enchanted creature gets +2/+2 and has flying" Aura-mode static.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
