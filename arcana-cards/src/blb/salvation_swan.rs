//! Salvation Swan — `{3}{W}` 3/3 Bird Cleric with Flash and Flying.
//! "Whenever this creature or another Bird you control enters, exile up to one
//!  target creature you control without flying. Return it to the battlefield
//!  under its owner's control with a flying counter on it at the beginning of
//!  the next end step."
//!
//! Modeled as a Bird-enters ZoneChange trigger that exiles the target and
//! schedules its return at the next end step. The "flying counter" rider on the
//! returned creature is GAP'd (the delayed return can't also stamp a counter).

use arcana_core::effects::{Effect, DelayedWhen, DelayedAction, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Salvation Swan");
    let bird = reg.interner_mut().intern("Bird");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };

    let bird_filter = script_bird_filter(reg);

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: bird_filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: exile_and_return,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .without_keyword(KeywordAbility::Flying),
                ),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

// Bird permanents you control (entering filter for the trigger).
fn script_bird_filter(reg: &mut CardRegistry) -> ObjectFilter {
    arcana_core::script::subtype_filter(reg, "Bird")
        .controlled_by(ControllerConstraint::You)
}

fn exile_and_return(
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
    vec![
        Effect::ExilePermanent { target: *id },
        // GAP: returned creature should re-enter "with a flying counter"; the
        // delayed return can't also add a counter to the (new) object.
        Effect::DelayedAction {
            source: *id,
            controller: trig.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnFromExileToBattlefield,
        },
    ]
}
