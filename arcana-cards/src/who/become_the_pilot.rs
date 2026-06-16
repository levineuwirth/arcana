//! Become the Pilot — `{3}{U}{U}` enchantment — Aura.
//! "Enchant noncommander creature. You control enchanted creature. Enchanted
//!  creature gets +2/+2 and can't be blocked unless it's attacking its owner or a
//!  permanent its owner controls."
//!
//! Control-change Aura. On ETB (id 1) the engine has already attached the Aura,
//! so the host is `source.attached_to`; we `ChangeControl` it to the Aura's
//! controller and install the +2/+2. When the Aura leaves (id 2) we revert
//! control to the host's owner. The conditional "can't be blocked unless …"
//! evasion is outside the demonstrated aura surface — GAP.

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
    let name = reg.interner_mut().intern("Become the Pilot");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    // NOTE: "noncommander" restriction approximated by caster's choice of Creature target.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
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
                effect: etb_gain_control,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfLeavesBattlefield,
                intervening_if: None,
                effect: leaves_revert_control,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_gain_control(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) else {
        return Vec::new();
    };
    // GAP: "can't be blocked unless it's attacking its owner or a permanent its
    // owner controls" — conditional evasion not expressible.
    vec![
        Effect::ChangeControl {
            target: host,
            new_controller: trig.controller,
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt(
                trig.source,
                2,
                2,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}

fn leaves_revert_control(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) else {
        return Vec::new();
    };
    let Some(owner) = state.object_or_lki(host).map(|o| o.owner) else {
        return Vec::new();
    };
    vec![Effect::ChangeControl {
        target: host,
        new_controller: owner,
    }]
}
