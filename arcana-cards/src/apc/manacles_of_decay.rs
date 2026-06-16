//! Manacles of Decay — `{1}{W}` enchantment — Aura.
//! "Enchant creature. Enchanted creature can't attack. {B}: Enchanted creature
//!  gets -1/-1 until end of turn. {R}: Enchanted creature can't block this
//!  turn."
//!
//! The static restriction is an ETB-installed `attached_cant_attack`. The two
//! granted host activated abilities are ETB-installed `attached_activated`
//! effects whose costs and effects run against the HOST: {B} pumps it -1/-1
//! until end of turn, {R} forbids it from blocking this turn.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Manacles of Decay");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_cant_attack(
                trig.source,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_activated(
                trig.source,
                ActivatedAbilityDef {
                    text: "{B}: Enchanted creature gets -1/-1 until end of turn."
                        .into(),
                    cost: ActivationCost {
                        mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                        ..Default::default()
                    },
                    target_requirements: Vec::new(),
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: debuff_host,
                },
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_activated(
                trig.source,
                ActivatedAbilityDef {
                    text: "{R}: Enchanted creature can't block this turn.".into(),
                    cost: ActivationCost {
                        mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                        ..Default::default()
                    },
                    target_requirements: Vec::new(),
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: forbid_host_block,
                },
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}

fn debuff_host(
    _state: &GameState,
    ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: -1,
        toughness: -1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn forbid_host_block(
    _state: &GameState,
    ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ForbidBlocking {
        target: ctx.source,
        duration: Duration::EndOfTurn,
    }]
}
