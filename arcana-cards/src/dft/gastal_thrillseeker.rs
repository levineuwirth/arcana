//! Gastal Thrillseeker — `{B}{R}` 2/3 Lizard Berserker.
//! Start your engines! (speed mechanic — unmodeled).
//! When this creature enters, it deals 1 damage to target opponent and you
//! gain 1 life.
//! Max speed — This creature has deathtouch and haste (conditional static
//! gated on speed 4).
//!
//! The "Start your engines!" / "Max speed" speed mechanic has no usable
//! KeywordAbility variant, and the Max-speed static (deathtouch+haste while
//! at speed 4) is a speed-gated continuous effect with no engine hook —
//! both GAP'd. The ETB damage + lifegain is wired.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gastal Thrillseeker");
    let lizard = reg.interner_mut().intern("Lizard");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(berserker);

    // GAP: keywords "Start your engines!" / "Max speed" (speed mechanic)
    // have no usable KeywordAbility variants.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP static: "Max speed — This creature has deathtouch and haste" is a
    // speed-gated continuous ability with no engine hook.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_ping_and_gain,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_opponent()],
        }),
    )
}

fn etb_ping_and_gain(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    if let Some(TargetChoice::Player(p)) = trig.targets.targets.first() {
        effects.push(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(*p),
            amount: 1,
        });
    }
    effects.push(Effect::GainLife {
        player: trig.controller,
        amount: 1,
    });
    effects
}
