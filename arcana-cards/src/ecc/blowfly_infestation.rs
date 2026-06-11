//! Blowfly Infestation — `{2}{B}` enchantment.
//! "Whenever a creature dies, if it had a -1/-1 counter on it, put a
//! -1/-1 counter on target creature."
//!
//! A dies-`ZoneChange` trigger. GAP: the intervening-if ("if it had a
//! -1/-1 counter on it") inspects the DYING creature's counters, which
//! no `conditions::` helper can express (`source_has_counter` checks
//! this enchantment, not the dying object) — the trigger fires
//! unconditionally.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blowfly Infestation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                // GAP: intervening-if "if it had a -1/-1 counter on it"
                // checks the dying creature's counters; no conditions::
                // helper reads a non-source object's counters, so the
                // trigger fires unconditionally.
                intervening_if: None,
                effect: spread_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                ],
            },
        ),
    )
}

/// "…put a -1/-1 counter on target creature."
fn spread_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::MinusOneMinusOne,
        count: 1,
    }]
}
