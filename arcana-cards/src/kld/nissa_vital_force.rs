//! Nissa, Vital Force — `{3}{G}{G}` Legendary Planeswalker — Nissa, starting loyalty 5.
//!
//! +1: Untap target land you control. Until your next turn, it becomes a 5/5
//!     Elemental creature with haste. It's still a land.
//! −3: Return target permanent card from your graveyard to your hand.
//! −6: You get an emblem with "Whenever a land you control enters, you may draw
//!     a card."
//!
//! # Scope
//! - `+1`: untaps a target land you control (implemented); the
//!   become-a-5/5-Elemental-with-haste-until-your-next-turn animation rider is
//!   a continuous self-transform not expressible with the demonstrated Effect
//!   surface — that part is GAP'd while the Untap still resolves.
//! - `−3`: "return target permanent card from your graveyard to your hand" is a
//!   graveyard-targeting effect with no any-graveyard target sentinel in the
//!   demonstrated surface — ability shell with correct `−3` cost, GAP'd body.
//! - `−6`: EMBLEM with a triggered ability ("Whenever a land you control enters,
//!   you may draw a card") — fully implemented as a triggered emblem
//!   (ZoneChange land-controlled-by-You enters → DrawCards 1).

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nissa, Vital Force");
    let nissa = reg.interner_mut().intern("Nissa");
    let _emblem = reg.interner_mut().intern("Nissa, Vital Force emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nissa);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Untap target land you control. Until your next turn, \
                       it becomes a 5/5 Elemental creature with haste. It's still \
                       a land."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types(TypeLine::LAND.into())
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_untap_land,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Return target permanent card from your graveyard to \
                       your hand."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_return,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: You get an emblem with \"Whenever a land you control \
                       enters, you may draw a card.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_emblem,
            }),
    )
}

/// `+1: Untap target land you control; it becomes a 5/5 Elemental with haste.`
fn plus_one_untap_land(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // The "becomes a 5/5 Elemental creature with haste until your next turn,
    // still a land" animation is a continuous self-transform not expressible
    // with the demonstrated Effect surface — GAP'd. The Untap still resolves.
    vec![Effect::Untap { target: *id }]
}

/// `−3: Return target permanent card from your graveyard to your hand.`
fn minus_three_return(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: graveyard-targeting return-to-hand; no any-graveyard target sentinel
    // in the demonstrated targeting surface.
    Vec::new()
}

/// `−6: You get an emblem with "Whenever a land you control enters, you may draw
/// a card."`
fn minus_six_emblem(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Nissa, Vital Force emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: emblem_draw,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

/// Emblem: "Whenever a land you control enters, you may draw a card."
fn emblem_draw(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
