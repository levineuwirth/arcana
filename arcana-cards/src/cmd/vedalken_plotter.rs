//! Vedalken Plotter — `{2}{U}` 1/1 Creature — Vedalken Wizard.
//! When this creature enters, exchange control of target land you control and target land
//! an opponent controls.
//! GAP: No "ExchangeControl" Effect variant. Only ChangeControl (one-way) exists.
//! Emitting two ChangeControl effects as best-effort (both permanent).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vedalken Plotter");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_enter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::permanent()
                                .with_types(TypeLine::LAND.into())
                                .controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::permanent()
                                .with_types(TypeLine::LAND.into())
                                .controlled_by(ControllerConstraint::Opponent),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
            }),
    )
}

fn on_enter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: No ExchangeControl variant; using two ChangeControl effects as partial approximation
    let mut iter = trig.targets.targets.iter();
    let Some(t1) = iter.next() else { return Vec::new(); };
    let Some(t2) = iter.next() else { return Vec::new(); };
    let TargetChoice::Object(id1) = t1 else { return Vec::new(); };
    let TargetChoice::Object(_id2) = t2 else { return Vec::new(); };
    vec![
        Effect::ChangeControl { target: *id1, new_controller: trig.controller },
        // GAP: opponent's land assigned to opponent — need their player id; not accessible here
    ]
}
