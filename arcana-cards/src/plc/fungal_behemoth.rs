//! Fungal Behemoth — `{3}{G}` */* Creature — Fungus.
//!
//! Oracle:
//! * Fungal Behemoth's power and toughness are each equal to the number of
//!   +1/+1 counters on creatures you control. (Installed at Layer 7a via an
//!   ETB self-CDA — `self_pt_cda` summing +1/+1 counters across the
//!   controller's creatures; symmetric `*`/`*`.)
//! * Suspend X—{X}{G}{G}. X can't be 0. — GAP: "Suspend" is not a usable
//!   `KeywordAbility` variant (suspend cast / time-counter mechanic unmodeled).
//! * Whenever a time counter is removed from this card while it's exiled,
//!   you may put a +1/+1 counter on target creature. — GAP: no
//!   counter-removed / exile-zone time-counter `TriggerCondition` variant.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fungal Behemoth");
    let fungus = reg.interner_mut().intern("Fungus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cda,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            plus_one_counters_on_your_creatures,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn plus_one_counters_on_your_creatures(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n: i32 = s
        .objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.is_creature() && o.controller == who)
        .map(|o| o.count_counters(CounterKind::PlusOnePlusOne) as i32)
        .sum();
    (n, n)
}
