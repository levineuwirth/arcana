//! Mistford River Turtle — `{3}{U}` 1/5 Creature — Turtle.
//! Whenever this creature attacks, another target attacking non-Human creature can't be blocked this turn.
//! Uses SelfAttacks trigger + targeted non-Human attacking creature + CantBeBlocked.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mistford River Turtle");
    let turtle = reg.interner_mut().intern("Turtle");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(turtle);
    // Build a non-Human attacking creature filter. We need to exclude creatures with Human subtype.
    // ObjectFilter doesn't have a direct "without subtype" refinement, so we use
    // creature() filter and note the Human-exclusion as a partial GAP.
    // The target is "another target attacking non-Human creature" — we model creature target;
    // GAP: non-Human restriction and "attacking" restriction not expressible in TargetFilter.
    let _ = human;
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: grant_unblockable,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    // GAP: "attacking non-Human creature" — no filter for "attacking" or "non-Human subtype"
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn grant_unblockable(
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
    vec![Effect::CantBeBlocked {
        target: *id,
        duration: Duration::EndOfTurn,
    }]
}
