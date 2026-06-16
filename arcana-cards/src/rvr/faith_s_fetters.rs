//! Faith's Fetters — `{3}{W}` enchantment — Aura.
//! "Enchant permanent.
//!  When this Aura enters, you gain 4 life.
//!  Enchanted permanent can't attack or block, and its activated
//!  abilities can't be activated unless they're mana abilities."
//!
//! On ETB the controller gains 4 life. The restriction is the Pacifism
//! pair (attached_cant_attack + attached_cant_block). The "activated
//! abilities can't be activated unless mana abilities" clause is a GAP
//! (no attached ability-lock builder).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Faith's Fetters");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Permanent(ObjectFilter::permanent()))
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
    // GAP: "its activated abilities can't be activated unless they're mana
    // abilities" (no attached ability-lock builder).
    vec![
        Effect::GainLife {
            player: trig.controller,
            amount: 4,
        },
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
    ]
}
