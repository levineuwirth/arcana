//! Rusting Golem — `{4}` */* Artifact Creature — Golem.
//! Fading 5 (enters with five fade counters; at the beginning of your
//! upkeep, remove a fade counter, else sacrifice). Synthesized from the
//! Fading keyword by the engine.
//! Rusting Golem's power and toughness are each equal to the number of
//! fade counters on it. (CDA — recorded as */* via PtValue::Star, wired at
//! Layer 7a via a `self_pt_cda` installed on ETB.)

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
    let name = reg.interner_mut().intern("Rusting Golem");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    // CDA "power and toughness each equal to the number of fade counters on
    // it" — recorded as */* via PtValue::Star, wired at Layer 7a by
    // install_cda on ETB.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Fading(5)],
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

fn install_cda(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

// P/T each equal to the number of fade counters on this creature.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let n = s
        .objects
        .get(source)
        .map(|o| o.count_counters(CounterKind::Fade))
        .unwrap_or(0) as i32;
    (n, n)
}
