//! Utopia Vow — `{1}{G}` enchantment — Aura.
//! "Enchant creature. Enchanted creature can't attack or block.
//!  Enchanted creature has \"{T}: Add one mana of any color.\""
//!
//! Restriction + host-activated mana Aura. ETB installs
//! `attached_cant_attack` + `attached_cant_block`, plus five
//! `attached_activated` mana abilities ({T}: add one mana of a fixed color,
//! one per WUBRG color — the shared tap cost means only one fires, modeling
//! "any color"). The cost and effect run against the HOST (`ctx.source`).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::{ManaCost, ManaUnit};
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
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Utopia Vow");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
            effect: ContinuousEffect::attached_cant_block(
                trig.source,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        host_mana_grant(trig.source, "{T}: Add {W}.", uv_add_white),
        host_mana_grant(trig.source, "{T}: Add {U}.", uv_add_blue),
        host_mana_grant(trig.source, "{T}: Add {B}.", uv_add_black),
        host_mana_grant(trig.source, "{T}: Add {R}.", uv_add_red),
        host_mana_grant(trig.source, "{T}: Add {G}.", uv_add_green),
    ]
}

fn host_mana_grant(
    source: arcana_core::objects::ObjectId,
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> Effect {
    Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_activated(
            source,
            ActivatedAbilityDef {
                text: text.into(),
                cost: ActivationCost {
                    tap: true,
                    ..Default::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect,
            },
            Duration::WhileSourceOnBattlefield,
        ),
    }
}

fn uv_add_one(ctx: &ActivationContext, color: ManaColor) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(color, ctx.source)],
    }]
}

fn uv_add_white(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    uv_add_one(ctx, ManaColor::White)
}
fn uv_add_blue(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    uv_add_one(ctx, ManaColor::Blue)
}
fn uv_add_black(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    uv_add_one(ctx, ManaColor::Black)
}
fn uv_add_red(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    uv_add_one(ctx, ManaColor::Red)
}
fn uv_add_green(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    uv_add_one(ctx, ManaColor::Green)
}
