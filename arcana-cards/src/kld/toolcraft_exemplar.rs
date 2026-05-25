//! Toolcraft Exemplar — `{W}` 1/1 white creature. "At the beginning of combat
//! on your turn, if you control an artifact, this creature gets +2/+1 until end
//! of turn. If you control three or more artifacts, it also gains first strike
//! until end of turn."
//!
//! GAP: intervening_if — "if you control an artifact" condition not
//! representable in TriggeredAbilityDef.intervening_if. Emitting full pump
//! with first strike unconditionally as best-effort.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Toolcraft Exemplar");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None, // GAP: intervening_if — "if you control an artifact"
                effect: combat_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn combat_pump(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    use arcana_core::script;
    let artifact_count = script::count_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    if artifact_count == 0 {
        return Vec::new();
    }
    let keywords = if artifact_count >= 3 {
        vec![KeywordAbility::FirstStrike]
    } else {
        vec![]
    };
    vec![Effect::Pump {
        target: trig.source,
        power: 2,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords,
    }]
}
