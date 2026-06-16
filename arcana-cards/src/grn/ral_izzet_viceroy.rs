//! Ral, Izzet Viceroy — `{3}{U}{R}` Legendary Planeswalker — Ral, starting loyalty 5.
//!
//! +1: Look at the top two cards of your library. Put one of them into your hand and
//!     the other into your graveyard. The look-and-split selection is not expressible
//!     from the demonstrated surface; modeled as a faithful net card-draw (Surveil 1
//!     then Draw 1) is also imperfect, so GAP the body. Shell kept with correct cost.
//! −3: Ral deals damage to target creature equal to the total number of instant and
//!     sorcery cards you own in exile and in your graveyard. Dynamic-X amount —
//!     GAP body, shell kept.
//! −8: You get an emblem with "Whenever you cast an instant or sorcery spell, this
//!     emblem deals 4 damage to any target and you draw two cards." Modeled via
//!     `CreateEmblem` with an instant/sorcery-cast TRIGGERED ability; the draw-two is
//!     modeled, the "4 damage to any target" rider has no target available from a
//!     triggered emblem fn — GAP that part.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ral, Izzet Viceroy");
    let ral = reg.interner_mut().intern("Ral");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ral);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Look at the top two cards of your library. Put one of \
                       them into your hand and the other into your graveyard."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Ral, Izzet Viceroy deals damage to target creature equal \
                       to the total number of instant and sorcery cards you own in \
                       exile and in your graveyard."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types(TypeLine::CREATURE.into()),
                    ),
                    count: arcana_core::targets::TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_dynamic,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: You get an emblem with \"Whenever you cast an instant or \
                       sorcery spell, this emblem deals 4 damage to any target and \
                       you draw two cards.\""
                    .into(),
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
                effect: minus_eight_emblem,
            }),
    )
}

fn plus_one_dig(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at top two, put one in hand and the other in graveyard" — the
    //      look-and-split-to-two-zones selection is not expressible from the
    //      demonstrated Effect surface.
    Vec::new()
}

fn minus_three_dynamic(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: dynamic X (damage equal to instant+sorcery cards in exile and graveyard).
    Vec::new()
}

fn minus_eight_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Ral, Izzet Viceroy").expect("name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter {
                        types: Some((TypeLine::INSTANT | TypeLine::SORCERY).into()),
                        ..Default::default()
                    }),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: emblem_draw_two,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

fn emblem_draw_two(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "this emblem deals 4 damage to any target" — no target is available from a
    //      triggered emblem fn here; the draw-two half is modeled.
    vec![Effect::DrawCards { player: trig.controller, count: 2 }]
}
