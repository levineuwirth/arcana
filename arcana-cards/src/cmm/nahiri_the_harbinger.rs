//! Nahiri, the Harbinger — `{2}{R}{W}` Legendary Planeswalker — Nahiri,
//! starting loyalty 3.
//!
//! +2: You may discard a card. If you do, draw a card.
//! −2: Exile target enchantment, tapped artifact, or tapped creature.
//! −8: Search your library for an artifact or creature card, put it onto
//!     the battlefield, then shuffle. It gains haste. Return it to your
//!     hand at the beginning of the next end step. (GAP — see below.)

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nahiri, the Harbinger");
    let nahiri = reg.interner_mut().intern("Nahiri");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nahiri);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // +2: You may discard a card. If you do, draw a card.
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: You may discard a card. If you do, draw a card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_loot,
            })
            // −2: Exile target enchantment, tapped artifact, or tapped creature.
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Exile target enchantment, tapped artifact, or tapped creature.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter {
                        custom: Some(is_enchantment_or_tapped_artifact_or_creature),
                        ..Default::default()
                    }),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_exile,
            })
            // −8: tutor an artifact/creature onto the battlefield with haste,
            //     bounce at next end step.
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: Search your library for an artifact or creature card, put it onto the battlefield, then shuffle. It gains haste. Return it to your hand at the beginning of the next end step.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_tutor,
            }),
    )
}

fn is_enchantment_or_tapped_artifact_or_creature(
    obj: &arcana_core::objects::GameObject,
    _state: &GameState,
) -> bool {
    let types = obj.characteristics.types;
    let tapped = obj.is_tapped();
    types.has(TypeLine::ENCHANTMENT)
        || (tapped && types.has(TypeLine::ARTIFACT))
        || (tapped && types.has(TypeLine::CREATURE))
}

fn plus_two_loot(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "You may discard a card. If you do, draw a card." — the "may" /
    // "if you do" optionality isn't expressible (OptionalPayment only
    // supports mana/life costs, not discard); approximated as a
    // mandatory discard-then-draw (rummage). On an empty hand the
    // Discard no-ops; the draw still happens — a documented fidelity gap.
    vec![Effect::Sequence(vec![
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards { player: ctx.controller, count: 1 },
    ])]
}

fn minus_two_exile(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ExilePermanent { target: *id }]
}

fn minus_eight_tutor(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Search for an artifact OR creature card, put onto battlefield, shuffle.
    // GAP: "It gains haste. Return it to your hand at the beginning of the
    //       next end step." — the tutored object's id is not visible to this
    //       resolver (TutorToBattlefield mints it internally), so the haste
    //       grant + delayed bounce cannot be attached.
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter {
            types_any: Some(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
            ..Default::default()
        },
        tapped: false,
    }]
}
