//! Pemmin's Aura — `{1}{U}{U}` enchantment — Aura.
//! "Enchant creature.
//!  {U}: Untap enchanted creature.
//!  {U}: Enchanted creature gains flying until end of turn.
//!  {U}: Enchanted creature gains shroud until end of turn.
//!  {1}: Enchanted creature gets +1/-1 or -1/+1 until end of turn."
//!
//! Four host-granted activated abilities installed on ETB via
//! attached_activated, each running against the enchanted creature
//! (ctx.source). The {1} ability is a modal "+1/-1 OR -1/+1"; activated
//! abilities have no modal support here, so it is fixed to +1/-1 and the
//! choice is noted as a GAP.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pemmin's Aura");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
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

fn ability(text: &str, cost: &str, effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost {
            mana_cost: ManaCost::parse(cost).unwrap(),
            tap: false,
            ..Default::default()
        },
        target_requirements: Vec::new(),
        is_mana_ability: false,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect,
    }
}

fn etb_install(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: the {1} ability's modal "+1/-1 OR -1/+1" choice — fixed to +1/-1.
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_activated(
                trig.source,
                ability("{U}: Untap enchanted creature.", "{U}", untap_host),
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_activated(
                trig.source,
                ability(
                    "{U}: Enchanted creature gains flying until end of turn.",
                    "{U}",
                    grant_flying_host,
                ),
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_activated(
                trig.source,
                ability(
                    "{U}: Enchanted creature gains shroud until end of turn.",
                    "{U}",
                    grant_shroud_host,
                ),
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_activated(
                trig.source,
                ability(
                    "{1}: Enchanted creature gets +1/-1 or -1/+1 until end of turn.",
                    "{1}",
                    pump_host,
                ),
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}

fn untap_host(_state: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Untap { target: ctx.source }]
}

fn grant_flying_host(_state: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Flying,
        duration: Duration::EndOfTurn,
    }]
}

fn grant_shroud_host(_state: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Shroud,
        duration: Duration::EndOfTurn,
    }]
}

fn pump_host(_state: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: -1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
