//! Tezzeret, Cruel Captain — `{3}` Legendary Planeswalker — Tezzeret,
//! starting loyalty 5.
//!
//! Whenever an artifact you control enters, put a loyalty counter on Tezzeret.
//! 0: Untap target artifact or creature. If it's an artifact creature, put a
//!   +1/+1 counter on it.
//! −3: Search your library for an artifact card with mana value 1 or less,
//!   reveal it, put it into your hand, then shuffle.
//! −7: You get an emblem with "At the beginning of combat on your turn, put
//!   three +1/+1 counters on target artifact you control. If it's not a
//!   creature, it becomes a 0/0 Robot artifact creature."
//!
//! GAP: the 0 ability's conditional "if it's an artifact creature, put a
//!   +1/+1 counter on it" rider is resolution-time-conditional on the target's
//!   types; the demonstrated Effect surface can't branch on the chosen
//!   target's characteristics, so only the unconditional Untap is emitted.
//! GAP: −7 grants an emblem; emblem creation is not in the demonstrated
//!   Effect surface. Ability shell declared with correct cost; effect GAP'd.
//! GAP: −3 tutor filter restricts to "mana value 1 or less" — TutorToHand's
//!   ObjectFilter surface shown here filters by type (artifact); the
//!   mana-value bound is not applied.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tezzeret, Cruel Captain");
    let sub = reg.interner_mut().intern("Tezzeret");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // Whenever an artifact you control enters, put a loyalty counter on Tezzeret.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::ARTIFACT.into())
                        .controlled_by(arcana_core::targets::ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: artifact_etb_loyalty,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // 0: Untap target artifact or creature. (+1/+1 rider GAP'd)
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Untap target artifact or creature. If it's an artifact creature, put a +1/+1 counter on it.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_untap,
            })
            // −3: tutor an artifact (MV bound GAP'd) to hand, revealed.
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Search your library for an artifact card with mana value 1 or less, reveal it, put it into your hand, then shuffle.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_tutor,
            })
            // −7: emblem (GAP)
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: You get an emblem with \"At the beginning of combat on your turn, put three +1/+1 counters on target artifact you control. If it's not a creature, it becomes a 0/0 Robot artifact creature.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_emblem,
            }),
    )
}

fn artifact_etb_loyalty(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Loyalty,
        count: 1,
    }]
}

fn zero_untap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: conditional "+1/+1 counter if it's an artifact creature" not
    // expressible on the chosen target's characteristics. Untap only.
    vec![Effect::Untap { target: *id }]
}

fn minus_three_tutor(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: mana-value <= 1 bound not applied; filtered to artifacts only.
    vec![Effect::TutorToHand {
        player: ctx.controller,
        filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
        reveal: true,
    }]
}

fn minus_seven_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem creation not in the demonstrated Effect surface.
    Vec::new()
}
