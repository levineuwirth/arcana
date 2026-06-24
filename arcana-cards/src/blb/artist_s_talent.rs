//! Artist's Talent — {1}{R} Enchantment — Class
//!
//! Level 1: Whenever you cast a noncreature spell, you may discard a card. If you do, draw a card.
//! {2}{R}: Level 2 — Noncreature spells you cast cost {1} less to cast.
//! {2}{R}: Level 3 — If a source you control would deal noncombat damage to an opponent or a
//!   permanent an opponent controls, it deals that much damage plus 2 instead.
//!
//! Level-1 "whenever you cast a noncreature spell, you may discard a card; if you do, draw a card"
//!      is wired as an OptionalPayment (Discard 1 → DrawCards 1). Per-level gating of the trigger
//!      is a separate known debt; Level 1 is the base level so the trigger is always active.
//! GAP: Level-2 static "noncreature spells you cast cost {1} less" — per-level cost-reduction
//!      continuous effect not in engine (continuous-effect engine subsystem).
//! GAP: Level-3 replacement "deals that much damage plus 2" — damage-replacement continuous effect
//!      not in engine (continuous-effect engine subsystem).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Artist's Talent");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Level,
                count: 1,
            })
            // Level 1: whenever you cast a noncreature spell, you may discard a
            // card. If you do, draw a card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: level1_loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}: Level 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").expect("valid cost"),
                    min_self_counters: Some((CounterKind::Level, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_2,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}: Level 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").expect("valid cost"),
                    min_self_counters: Some((CounterKind::Level, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_3,
            })
    )
}

fn level1_loot(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "You may discard a card. If you do, draw a card."
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Discard(1),
        then: Box::new(Effect::DrawCards { player: trig.controller, count: 1 }),
        else_effect: None,
    }]
}

fn level_up_to_2(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Level-2 static "noncreature spells you cast cost {1} less" not modeled
    //      (cost-reduction continuous effect not in engine).
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Level,
            count: 1,
        },
    ]
}

fn level_up_to_3(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Level-3 static "noncombat damage +2" replacement not modeled
    //      (damage-replacement continuous effect not in engine).
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Level,
            count: 1,
        },
    ]
}
