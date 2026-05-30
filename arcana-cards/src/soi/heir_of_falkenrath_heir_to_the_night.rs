//! Heir of Falkenrath // Heir to the Night — `{1}{B}` Vampire 2/1 (front) /
//! Vampire Berserker (back, Flying).
//!
//! Front face:
//!   Discard a card: Transform this creature. Activate only once each turn.
//!
//! Back face (Heir to the Night):
//!   Flying
//!
//! GAP: "Activate only once each turn" enforcement is engine debt (not modeled
//!      in ActivationCost); the ability fires each time the cost is paid.
//! GAP: back-face-only triggered abilities not auto-installed on transform.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Heir of Falkenrath");
    let vampire_sub = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: Heir to the Night — Vampire Berserker with Flying
    let back_name = reg.interner_mut().intern("Heir to the Night");
    let vampire_back = reg.interner_mut().intern("Vampire");
    let berserker_back = reg.interner_mut().intern("Berserker");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vampire_back);
    back_subtypes.0.insert(berserker_back);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
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
            // "Discard a card: Transform this creature. Activate only once each turn."
            // GAP: "only once each turn" not enforced by engine; modeled as an activated ability.
            .with_activated_ability(ActivatedAbilityDef {
                text: "Discard a card: Transform this creature. Activate only once each turn."
                    .into(),
                cost: ActivationCost {
                    discard_other: Some(ObjectFilter::new()),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(0), // front face only
                effect: transform_to_night,
            }),
    )
}

fn transform_to_night(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
