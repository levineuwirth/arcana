//! Tidewalker — {2}{U} */* Elemental.
//! Enters with a time counter for each Island you control. Vanishing.
//! Power and toughness each equal the number of time counters on it.
//!
//! The */* CDA ("equal to the number of time counters on it") is wired at
//! Layer 7a via `ContinuousEffect::self_pt_cda` on a
//! `SelfEntersBattlefield` trigger (compute reads the source's own Time
//! counters — a recursion-proof scalar). `PtValue::Star` kept as bones.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Tidewalker");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        // Vanishing has no printed count ("Vanishing"); but it actually uses
        // time counters seeded per-Island, modeled as Vanishing(1) fallback.
        keywords: vec![KeywordAbility::Vanishing(1)],
        ..Default::default()
    };

    // GAP: "enters with a time counter for each Island you control" — no
    //      enters-with-N-counters static rider expressible here.
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

/// "Tidewalker's power and toughness are each equal to the number of time
/// counters on it" — install the self-CDA at Layer 7a.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            time_counter_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Power and toughness each equal to the number of time counters on this.
fn time_counter_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let n = s
        .objects
        .get(source)
        .map(|o| o.count_counters(CounterKind::Time))
        .unwrap_or(0) as i32;
    (n, n)
}
