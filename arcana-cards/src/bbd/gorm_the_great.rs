//! Gorm the Great — `{3}{G}` 2/7 Legendary Giant Warrior.
//! Partner with Virtus the Veiled; Vigilance; "Gorm must be blocked if able,
//! and Gorm must be blocked by two or more creatures if able."
//!
//! Vigilance is a base keyword. Partner-with's ETB tutor-helper and the
//! must-be-blocked combat-modifier statics are not expressible with the
//! demonstrated API and are GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gorm the Great");
    let giant = reg.interner_mut().intern("Giant");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: Partner with Virtus (ETB tutor-helper for a specifically-named partner) is not expressible.
    // GAP: "must be blocked if able / must be blocked by two or more creatures" is a static combat-requirement modifier with no Effect/Trigger representation.
    reg.register(CardDefinition::new(name, chars))
}
