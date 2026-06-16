//! Tamiyo, the Moon Sage — `{3}{U}{U}` Legendary Planeswalker — Tamiyo, starting loyalty 4.
//!
//! +1: Tap target permanent. It doesn't untap during its controller's
//!   next untap step. Modeled as `Effect::Tap`; the "doesn't untap next
//!   untap step" rider isn't expressible and is GAP'd.
//! −2: Draw a card for each tapped creature target player controls. GAP:
//!   dynamic-X draw (count = tapped creatures) isn't expressible; the
//!   ability shell carries the correct −2 cost.
//! −8: You get an emblem with "You have no maximum hand size" and
//!   "Whenever a card is put into your graveyard from anywhere, you may
//!   return it to your hand." The emblem is created with the
//!   graveyard-entry trigger, but: the no-maximum-hand-size static is a
//!   rule-altering grant no builder expresses, and the reflexive "return
//!   it" references the specific moved card (not exposed) — both GAP'd.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tamiyo, the Moon Sage");
    let tamiyo = reg.interner_mut().intern("Tamiyo");
    let _emblem = reg.interner_mut().intern("Tamiyo, the Moon Sage emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tamiyo);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Tap target permanent. It doesn't untap during its \
                       controller's next untap step.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::new()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_tap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Draw a card for each tapped creature target player \
                       controls.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: You get an emblem with \"You have no maximum hand \
                       size\" and \"Whenever a card is put into your \
                       graveyard from anywhere, you may return it to your \
                       hand.\"".into(),
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

fn plus_one_tap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "doesn't untap during its controller's next untap step" rider
    // isn't expressible; the tap itself is.
    vec![Effect::Tap { target: *id }]
}

fn minus_two_draw(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: dynamic-X draw (one card per tapped creature the target player
    // controls) isn't expressible.
    Vec::new()
}

fn minus_eight_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Tamiyo, the Moon Sage emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            // GAP: "no maximum hand size" is a rule-altering static no
            // builder expresses, so it is omitted from statics.
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new().controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: emblem_recur,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

fn emblem_recur(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return IT to your hand" references the specific card that was
    // just put into the graveyard — the moved object isn't exposed to the
    // trigger effect for a reflexive return.
    Vec::new()
}
