//! Spawnbroker — `{2}{U}` 1/1 blue Human Wizard. "When this creature
//! enters, you may exchange control of target creature you control and
//! target creature with power less than or equal to that creature's
//! power an opponent controls."
//!
//! Modeled as an ETB trigger with two creature targets (one you
//! control, one an opponent controls). The exchange is expressed as a
//! pair of `Effect::ChangeControl`: your creature goes to the
//! opponent's controller and theirs comes to you. There is no single
//! "exchange control" catalog variant, so this two-`ChangeControl`
//! swap is the faithful composition.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spawnbroker");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: exchange_control,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
            ],
        }),
    )
}

fn exchange_control(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: power-comparison restriction (opponent's creature power <= yours)
    // is not expressible in the target filter and isn't re-checked here.
    let Some(TargetChoice::Object(mine)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(theirs)) = trig.targets.targets.get(1) else {
        return Vec::new();
    };
    let their_controller = script::target_controller(state, *theirs, trig.controller);
    vec![
        Effect::ChangeControl {
            target: *mine,
            new_controller: their_controller,
        },
        Effect::ChangeControl {
            target: *theirs,
            new_controller: trig.controller,
        },
    ]
}
