//! A-Asari Captain — `{1}{R}{W}` 2/1 red/white Human Samurai with Trample and Haste.
//! "Whenever a Samurai or Warrior you control attacks alone, it gets +1/+0 until end
//! of turn for each Samurai or Warrior you control."
//! Trigger via TriggerCondition::AttacksAlone over Samurai-or-Warrior you control;
//! the lone attacker (trig.lone_attacker()) gets a dynamic +N/+0 where N is the count
//! of Samurai or Warrior creatures you control.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Asari Captain");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::AttacksAlone {
                filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .with_subtypes_any(vec![samurai, warrior]),
            },
            intervening_if: None,
            effect: on_attacks_alone,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_attacks_alone(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(attacker) = trig.lone_attacker() else {
        return Vec::new();
    };
    let samurai = reg.interner().lookup("Samurai").unwrap_or_default();
    let warrior = reg.interner().lookup("Warrior").unwrap_or_default();
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![samurai, warrior]);
    let n = script::count_matching(state, &filter, trig.controller);
    vec![Effect::Pump {
        target: attacker,
        power: n as i32,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
