//! Lotus Guardian — `{7}` 4/4 Artifact Creature — Dragon with Flying.
//! {T}: Add one mana of any color.
//!
//! "Add one mana of any color" requires a color-choice prompt that has no
//! expressible primitive (AddMana needs explicit ManaUnits; there is no
//! any-color / choose-a-color mana primitive in the usable surface). GAP'd;
//! Flying retained.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lotus Guardian");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "{T}: Add one mana of any color" — no any-color / color-choice mana primitive.
    reg.register(CardDefinition::new(name, chars))
}
