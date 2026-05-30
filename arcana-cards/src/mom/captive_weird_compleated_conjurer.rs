//! Captive Weird // Compleated Conjurer — `{U}` Weird creature 1/3 with
//! Defender. Front face: `{3}{R/P}`: Transform this creature (sorcery speed).
//! Back face (Compleated Conjurer): When this creature transforms into
//! Compleated Conjurer, exile the top card of your library; until end of your
//! next turn you may play that card.
//!
//! GAP: The activated cost `{R/P}` (hybrid Phyrexian mana) is not expressible
//! via `ManaCost::parse` — the engine does not model hybrid-Phyrexian symbols.
//! The activated ability is omitted entirely.
//! GAP: Back-face-only triggered ability not modeled (the transform-into trigger
//! would need face-gate support on TriggeredAbilityDef). "Until end of your next
//! turn you may play that card" is also not expressible.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Captive Weird");
    let weird_sub = reg.interner_mut().intern("Weird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(weird_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Compleated Conjurer");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let weird_back_sub = reg.interner_mut().intern("Weird");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(phyrexian_sub);
    back_subtypes.0.insert(weird_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: front-face activated ability {3}{R/P}: Transform — Phyrexian hybrid
    // cost not parseable; ability omitted.
    // GAP: back-face transform-into trigger ("When this creature transforms into
    // Compleated Conjurer, exile the top card of your library; until end of your
    // next turn you may play that card") — back-face-only triggered ability not
    // modeled.

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}
