//! Nesting Bot — `{W}` 1/1 Robot Artifact Creature.
//! "Start your engines!"
//! "When this creature dies, create a 1/1 colorless Servo artifact creature token."
//! "Max speed — This creature gets +1/+0."
//!
//! GAP: "Start your engines!" and "Max speed" are not usable `KeywordAbility`
//! variants (the speed mechanic is unmodeled) — keywords omitted.
//! GAP: "Max speed — This creature gets +1/+0." is a speed-gated static; omitted.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nesting Bot");
    let robot = reg.interner_mut().intern("Robot");
    let _servo = reg.interner_mut().intern("Servo");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: dies_make_servo,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dies_make_servo(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let servo = reg.interner().lookup("Servo").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(servo);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: servo,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
