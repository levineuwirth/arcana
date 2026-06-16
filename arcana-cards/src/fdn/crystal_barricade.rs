//! Crystal Barricade — `{1}{W}` 0/4 Artifact Creature — Wall with Defender.
//! "You have hexproof."
//! "Prevent all noncombat damage that would be dealt to other creatures you
//! control."
//!
//! Defender is a base keyword. The "you have hexproof" player-static and the
//! board-wide noncombat-damage prevention static both have no triggered/
//! activated form expressible in this class — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crystal Barricade");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: static "You have hexproof" (player-level static keyword grant).
    // GAP: static "Prevent all noncombat damage to other creatures you control"
    // (continuous board-wide prevention replacement).
    reg.register(CardDefinition::new(name, chars))
}
