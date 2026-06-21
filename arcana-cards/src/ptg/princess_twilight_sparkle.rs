//! Princess Twilight Sparkle — `{W}{U}` 2/2 Legendary Creature — Alicorn
//! with Flying.
//!
//! Oracle text:
//! * Flying — base keyword.
//! * "Other Alicorns, Horses, Pegasi, Ponies, and Unicorns you control get
//!   +1/+1." — GAP: a static continuous anthem (multi-subtype "other … you
//!   control get +X/+X") is not expressible on the MultiAbilityCreature shape.
//! * "{W}{U}{B}{R}{G}: If you control Applejack, Fluttershy, Pinkie Pie,
//!   Rainbow Dash, and Rarity, everypony wins the game." — GAP: there is no
//!   win-the-game Effect, and the five-named-creature activation precondition
//!   has no condition predicate; the activated ability is omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Princess Twilight Sparkle");
    let alicorn = reg.interner_mut().intern("Alicorn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alicorn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static anthem "Other Alicorns/Horses/Pegasi/Ponies/Unicorns you
    // control get +1/+1" — static continuous, not expressible.
    // GAP: "{W}{U}{B}{R}{G}: … everypony wins the game" — no win-the-game
    // Effect and no named-creature precondition predicate.
    reg.register(CardDefinition::new(name, chars))
}
