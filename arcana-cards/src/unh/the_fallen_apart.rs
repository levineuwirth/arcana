//! The Fallen Apart — `{2}{B}{B}` 4/4 Zombie (black).
//! "This creature enters with two arms and two legs."
//! "Whenever damage is dealt to this creature, remove an arm or a leg from it." (GAP — choice)
//! "This creature can't attack if it has no legs and can't block if it has no arms." (GAP — static)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP (static): "can't attack if it has no legs and can't block if it has no
// arms" — a counter-conditioned attack/block restriction static, not
// expressible in this card class.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Fallen Apart");
    let zombie = reg.interner_mut().intern("Zombie");
    let arm = reg.interner_mut().intern("arm");
    let leg = reg.interner_mut().intern("leg");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let _ = (arm, leg);

    reg.register(
        CardDefinition::new(name, chars)
            // "Enters with two arms and two legs" → two arm + two leg counters.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: add_limb_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "Whenever damage is dealt to this creature, remove an arm or a leg
            // from it." — fires the trigger, but the arm-or-leg choice is not
            // expressible (no choose-which-counter primitive).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfIsDealtDamage { combat_only: false },
                intervening_if: None,
                effect: remove_a_limb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_limb_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut out = Vec::new();
    if let Some(arm) = reg.interner().lookup("arm").map(CounterKind::Named) {
        out.push(Effect::AddCounters { target: trig.source, kind: arm, count: 2 });
    }
    if let Some(leg) = reg.interner().lookup("leg").map(CounterKind::Named) {
        out.push(Effect::AddCounters { target: trig.source, kind: leg, count: 2 });
    }
    out
}

fn remove_a_limb(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "remove an arm or a leg" — the controller chooses which of two named
    // counter kinds to remove; no choose-between-counter-kinds primitive exists.
    Vec::new()
}
