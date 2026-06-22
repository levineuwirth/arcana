//! Predation Steward — `{1}{G}` 2/2 Phyrexian Elf Warrior (G).
//! "This creature enters with two oil counters on it."
//! `{2}{G}, {T}, Remove an oil counter from this creature: Target creature
//! gets +2/+2 until end of turn. Activate only as a sorcery.`
//!
//! The "enters with two oil counters" static is modeled as an ETB trigger that
//! adds two oil (Named) counters. The activated ability pays mana + tap +
//! removes one oil counter; sorcery-speed is the default (is_instant_speed:
//! false).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Predation Steward");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let oil = reg.interner_mut().intern("oil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_two_oil,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G}, {T}, Remove an oil counter from this creature: \
                       Target creature gets +2/+2 until end of turn. Activate \
                       only as a sorcery."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}").expect("valid cost"),
                    tap: true,
                    remove_self_counter: Some((CounterKind::Named(oil), 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_target,
            }),
    )
}

fn etb_two_oil(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(oil) = reg.interner().lookup("oil") else { return Vec::new(); };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Named(oil),
        count: 2,
    }]
}

fn pump_target(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Pump {
        target: *id,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
