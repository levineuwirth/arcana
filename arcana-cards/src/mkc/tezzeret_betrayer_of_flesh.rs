//! Tezzeret, Betrayer of Flesh — `{2}{U}{U}` Legendary Planeswalker — Tezzeret, starting loyalty 5.
//!
//! Static (not a loyalty ability): "The first activated ability of an
//!   artifact you activate each turn costs {2} less." GAP: a passive
//!   cost-reduction static, not expressible from the loyalty surface.
//! +1: Draw two cards. Then discard two cards unless you discard an
//!   artifact card. Modeled as draw two, discard two; the "unless you
//!   discard an artifact" conditional is GAP'd.
//! −2: Target artifact becomes an artifact creature; if it isn't a
//!   Vehicle it has base power and toughness 4/4. Modeled as `SetBasePT`
//!   4/4 + `AddType` creature on the target artifact; the Vehicle
//!   exception is GAP'd (approximated by always setting 4/4).
//! −6: You get an emblem with "Whenever an artifact you control becomes
//!   tapped, draw a card." Fully implemented as a triggered emblem.

use arcana_core::effects::{DiscardChoice, Effect, EmblemDefinition};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Tezzeret, Betrayer of Flesh");
    let tezzeret = reg.interner_mut().intern("Tezzeret");
    let _emblem = reg.interner_mut().intern("Tezzeret, Betrayer of Flesh emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tezzeret);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Draw two cards. Then discard two cards unless you \
                       discard an artifact card.".into(),
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
                effect: plus_one_loot,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Target artifact becomes an artifact creature. If \
                       it isn't a Vehicle, it has base power and toughness \
                       4/4.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_animate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: You get an emblem with \"Whenever an artifact you \
                       control becomes tapped, draw a card.\"".into(),
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

fn plus_one_loot(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "unless you discard an artifact card" conditional skip isn't
    // expressible; model draw two, discard two.
    vec![
        Effect::DrawCards { player: ctx.controller, count: 2 },
        Effect::Discard {
            player: ctx.controller,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}

fn minus_two_animate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: the "if it isn't a Vehicle" exception is not expressible;
    // approximate by always setting base 4/4 and adding the creature type.
    vec![
        Effect::AddType { target: *id, types: TypeLine::CREATURE.into(), duration: Duration::Permanent },
        Effect::SetBasePT { target: *id, power: 4, toughness: 4, duration: Duration::Permanent },
    ]
}

fn minus_six_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Tezzeret, Betrayer of Flesh emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::BecomesTapped {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::ARTIFACT.into())
                        .controlled_by(ControllerConstraint::You),
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

fn emblem_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
