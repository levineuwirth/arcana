//! Chorus of the Tides — `{3}{U}` 3/2 Siren with Flying.
//! Flying.
//! Heroic — Whenever you cast a spell that targets this creature, scry 1.
//!
//! Flying is a base characteristic. Heroic is modeled the canonical way:
//! `SelfBecomesTarget { caster: You }` (fires when a spell you cast targets
//! this creature) → `Effect::Scry { count: 1 }` (Akroan Skyguard idiom).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chorus of the Tides");
    let siren = reg.interner_mut().intern("Siren");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siren);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfBecomesTarget {
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: heroic_scry,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn heroic_scry(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Scry {
        player: trig.controller,
        count: 1,
    }]
}
