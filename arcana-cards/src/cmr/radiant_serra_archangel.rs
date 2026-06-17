//! Radiant, Serra Archangel — `{6}{W}` 6/4 Legendary Angel with Flying.
//! "Tap another untapped creature you control with flying: Radiant gains
//!  protection from the color of your choice until end of turn.
//!  Partner."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Radiant, Serra Archangel");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Partner is not in the usable KeywordAbility surface; omitted.
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Tap another untapped creature you control with flying: Radiant gains
    // protection from the color of your choice until end of turn" — the effect
    // (gain protection from a chosen color) is not expressible (no protection
    // effect, no color-choice mechanism), so the whole activated ability is
    // omitted.
    reg.register(CardDefinition::new(name, chars))
}
