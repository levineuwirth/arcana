//! Toolcraft Exemplar — `{W}` 1/1 white creature. "At the beginning of combat
//! on your turn, if you control an artifact, this creature gets +2/+1 until end
//! of turn. If you control three or more artifacts, it also gains first strike
//! until end of turn."
//!
//! Intervening-if "if you control an artifact" modeled via
//! `conditions::you_control_a` on `intervening_if` (the trigger no longer
//! goes on the stack when you control no artifact, per CR 603.4); the
//! resolution-time count still gates the first-strike upgrade (≥3 artifacts).

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::objects::ObjectId;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
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
                intervening_if: Some(iif_control_artifact),
                effect: combat_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn iif_control_artifact(state: &GameState, _source: ObjectId, you: PlayerId) -> bool {
    conditions::you_control_a(state, you, &ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()))
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

#[cfg(test)]
mod tests {
    use super::*;
    use arcana_core::objects::GameObject;
    use arcana_core::zones::Zone;

    fn put_artifact(s: &mut GameState, controller: PlayerId) {
        let id = s.allocate_object_id();
        let chars = Characteristics {
            types: TypeLine::ARTIFACT.into(),
            ..Default::default()
        };
        let mut o = GameObject::new(id, controller, Zone::Battlefield, 0, chars);
        o.controller = controller;
        s.objects.insert(o);
    }

    // The intervening-if must gate the combat-begins trigger on "you control
    // an artifact" — the whole point of wiring it (the trigger no longer goes
    // on the stack when the condition is false).
    #[test]
    fn intervening_if_gates_on_controlling_an_artifact() {
        let mut s = GameState::new(2, 0);
        // No artifacts → must not fire.
        assert!(!iif_control_artifact(&s, 1, 0));
        // An artifact YOU control → fires.
        put_artifact(&mut s, 0);
        assert!(iif_control_artifact(&s, 1, 0));
        // Only an OPPONENT's artifact → still must not fire for you.
        let mut s2 = GameState::new(2, 0);
        put_artifact(&mut s2, 1);
        assert!(!iif_control_artifact(&s2, 1, 0));
    }
}
