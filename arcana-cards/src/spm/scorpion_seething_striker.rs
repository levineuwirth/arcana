//! Scorpion, Seething Striker — `{3}{B}` 3/3 Legendary Scorpion Human Villain.
//! Deathtouch. At the beginning of your end step, if a creature died this turn,
//! target creature you control connives.
//!
//! Deathtouch is a base keyword. The end-step trigger is wired with an
//! intervening-if (a creature died this turn) and targets a creature you
//! control. The connive payload (draw, discard, conditional +1/+1 counter) has
//! no `Effect` variant, so the trigger fires but its effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scorpion, Seething Striker");
    let scorpion = reg.interner_mut().intern("Scorpion");
    let human = reg.interner_mut().intern("Human");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scorpion);
    subtypes.0.insert(human);
    subtypes.0.insert(villain);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: keyword Connive is not in the usable keyword surface for this class.
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_a_creature_died),
                effect: connive_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn if_a_creature_died(s: &GameState, _src: ObjectId, _you: arcana_core::types::PlayerId, _reg: &CardRegistry) -> bool {
    script::creatures_died_this_turn(s) >= 1
}

fn connive_target(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: Connive (draw, discard, conditional +1/+1 counter on the conniving
    // creature) has no Effect variant in the usable surface.
    Vec::new()
}
