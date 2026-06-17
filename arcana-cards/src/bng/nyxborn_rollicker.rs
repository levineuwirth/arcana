//! Nyxborn Rollicker — `{R}` 1/1 Enchantment Creature — Satyr.
//! Bestow {1}{R}; "Enchanted creature gets +1/+1."
//!
//! Both lines are unexpressible with the demonstrated creature API:
//! Bestow is not an available KeywordAbility variant, and "Enchanted
//! creature gets +1/+1" is a static aura continuous ability, not a
//! triggered/activated ability. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nyxborn Rollicker");
    let satyr = reg.interner_mut().intern("Satyr");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(satyr);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Bestow {1}{R} — not an available KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "Enchanted creature gets +1/+1." — a static aura continuous
    // ability; not expressible as a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
