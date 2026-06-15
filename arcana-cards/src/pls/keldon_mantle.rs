//! Keldon Mantle — `{1}{R}` enchantment — Aura.
//! "Enchant creature. {B}: Regenerate enchanted creature. {R}: Enchanted
//!  creature gets +1/+0 until end of turn. {G}: Enchanted creature gains
//!  trample until end of turn."
//!
//! Three host-granted activated abilities, each installed via
//! `attached_activated` so the cost and effect run against the HOST
//! (`ctx.source`): {B} regenerate, {R} +1/+0, {G} trample.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Keldon Mantle");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
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
            effect: ContinuousEffect::attached_activated(
                trig.source,
                ActivatedAbilityDef {
                    text: "{B}: Regenerate enchanted creature.".into(),
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
                    effect: regen_host,
                },
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_activated(
                trig.source,
                ActivatedAbilityDef {
                    text: "{R}: Enchanted creature gets +1/+0 until end of turn."
                        .into(),
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
                    effect: pump_host,
                },
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_activated(
                trig.source,
                ActivatedAbilityDef {
                    text: "{G}: Enchanted creature gains trample until end of turn."
                        .into(),
                    cost: ActivationCost {
                        mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                        ..Default::default()
                    },
                    target_requirements: Vec::new(),
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: trample_host,
                },
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}

fn regen_host(
    _state: &GameState,
    ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Regenerate { target: ctx.source }]
}

fn pump_host(
    _state: &GameState,
    ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn trample_host(
    _state: &GameState,
    ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Trample,
        duration: Duration::EndOfTurn,
    }]
}
