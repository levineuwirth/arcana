//! One-Eyed Scarecrow — `{3}` 2/3 Artifact Creature — Scarecrow with
//! Defender. Creatures with flying your opponents control get -1/-0
//! (static — GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("One-Eyed Scarecrow");
    let scarecrow = reg.interner_mut().intern("Scarecrow");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scarecrow);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: "Creatures with flying your opponents control get -1/-0." is
    // a static continuous board-wide debuff, not a triggered/activated
    // ability.
    reg.register(CardDefinition::new(name, chars))
}
