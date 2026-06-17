//! Iroas, God of Victory — `{2}{R}{W}` 7/4 Legendary Enchantment Creature — God,
//! with Indestructible. The three remaining lines are pure static abilities with no
//! expressible primitive on the MultiAbilityCreature surface, so they are GAP'd:
//!   - "As long as your devotion to red and white is less than seven, Iroas isn't a
//!     creature." (devotion-gated type removal — static)
//!   - "Creatures you control have menace." (anthem-style keyword grant — static)
//!   - "Prevent all damage that would be dealt to attacking creatures you control."
//!     (continuous replacement — static)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Iroas, God of Victory");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Indestructible],
        ..Default::default()
    };

    // GAP: static "devotion < 7 → isn't a creature" — no expressible primitive.
    // GAP: static "Creatures you control have menace." — anthem, no primitive.
    // GAP: static "Prevent all damage to attacking creatures you control." — no primitive.
    reg.register(CardDefinition::new(name, chars))
}
