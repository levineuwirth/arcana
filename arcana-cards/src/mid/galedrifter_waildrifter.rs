//! Galedrifter // Waildrifter — `{3}{U}` Creature — Hippogriff 3/2 (front) /
//! Creature — Hippogriff Spirit (back). Transform/Disturb.
//!
//! Front: Flying. Disturb {4}{U} (You may cast this card from your graveyard transformed.)
//! Back: Flying. If Waildrifter would be put into a graveyard from anywhere, exile it instead.
//!
//! GAP: Disturb keyword not modeled (cast from graveyard transformed — no engine variant).
//!   No transform trigger wired; card is registered front-face only.
//! GAP: Back face "exile instead of graveyard" replacement effect not modeled.
//! GAP: Back-face-only triggered/replacement abilities not auto-installed on transform.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Galedrifter");

    let hippogriff_sub = reg.interner_mut().intern("Hippogriff");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hippogriff_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Waildrifter");
    let back_hippogriff_sub = reg.interner_mut().intern("Hippogriff");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_hippogriff_sub);
    back_subtypes.0.insert(spirit_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
        // GAP: Disturb — cast from graveyard transformed; no engine support.
        // GAP: Back face "exile instead of graveyard" replacement effect not modeled.
    )
}
