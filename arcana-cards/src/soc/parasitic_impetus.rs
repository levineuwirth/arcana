//! Parasitic Impetus — `{2}{B}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +2/+2 and is goaded. Whenever
//!  enchanted creature attacks, its controller loses 2 life and you gain 2
//!  life."
//!
//! +2/+2 is an ETB-installed attached_pt. The host-attacks payoff is an
//! `AttachedCreatureDoes { SelfAttacks }` trigger that drains the host's
//! controller and gains the Aura's controller life. Goad is not in the
//! usable keyword surface, so the "is goaded" clause is omitted.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Parasitic Impetus");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
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
                effect: etb_install_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfAttacks),
                },
                intervening_if: None,
                effect: on_host_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_pump(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: "is goaded" — Goad is not in the usable keyword surface.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            2,
            2,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn on_host_attacks(state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) else {
        return Vec::new();
    };
    let Some(host_controller) = state.object_or_lki(host).map(|o| o.controller) else {
        return Vec::new();
    };
    vec![
        Effect::LoseLife {
            player: host_controller,
            amount: 2,
        },
        Effect::GainLife {
            player: trig.controller,
            amount: 2,
        },
    ]
}
