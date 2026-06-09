//! Akroan Hoplite — `{R}{W}` 1/2 red-white Creature — Human Soldier.
//! "Whenever this creature attacks, it gets +X/+0 until end of turn, where
//! X is the number of attacking creatures you control."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Akroan Hoplite");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attacks_pump_x,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attacks_pump_x(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::count_matching(
        state,
        &ObjectFilter::creature()
            .attacking_only()
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    ) as i32;
    vec![Effect::Pump {
        target: trig.source,
        power: x,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
