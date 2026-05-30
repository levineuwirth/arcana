//! Baithook Angler // Hook-Haunt Drifter — `{1}{U}` Human Peasant creature 2/1.
//! Front face: Disturb {1}{U} (cast from graveyard transformed for disturb cost).
//! Back face (Hook-Haunt Drifter): Flying Spirit creature. If it would be put
//! into a graveyard from anywhere, exile it instead.
//!
//! # GAPs
//! - Disturb keyword (cast from graveyard transformed) is not in the engine's
//!   keyword surface; it is omitted from the front-face keywords.
//! - "If Hook-Haunt Drifter would be put into a graveyard from anywhere, exile
//!   it instead" is a replacement effect not expressible via the current engine
//!   API; emitted as GAP comment.
//! - Transform keyword is not in the keyword surface (it's a layout marker);
//!   omitted.
//! - The back face's own triggered/replacement abilities are not auto-installed
//!   on transform — GAP: back-face-only replacement ability not modeled.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::effects::KeywordAbility;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Baithook Angler");
    let human_sub = reg.interner_mut().intern("Human");
    let peasant_sub = reg.interner_mut().intern("Peasant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(peasant_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        // GAP: Disturb {1}{U} not in engine keyword surface.
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Hook-Haunt Drifter");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(spirit_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            // GAP: back-face-only replacement ability not modeled
            // ("If this would be put into a graveyard, exile it instead")
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
