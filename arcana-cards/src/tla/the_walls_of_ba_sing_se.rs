//! The Walls of Ba Sing Se — `{8}` 0/30 Legendary Artifact Creature — Wall.
//!
//! Defender
//! Other permanents you control have indestructible.
//!
//! Decomposition: Defender → `keywords`. "Other permanents you control
//! have indestructible" is a pure static continuous ability (no trigger
//! word, no cost) granting a keyword board-wide — not expressible as a
//! triggered/activated ability, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Walls of Ba Sing Se");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(30)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };
    // GAP: static "Other permanents you control have indestructible." — a
    // board-wide keyword-granting continuous ability with no trigger/cost;
    // not expressible as a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
