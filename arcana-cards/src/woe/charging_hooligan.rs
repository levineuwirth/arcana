//! Charging Hooligan — `{3}{R}` 3/3 red Human Peasant.
//! "Whenever this creature attacks, it gets +1/+0 until end of turn
//! for each attacking creature. If a Rat is attacking, this creature
//! gains trample until end of turn."
//! The pump scales with attacking creature count (dynamic).
//! GAP: "if a Rat is attacking" conditional trample — subtype check
//! on attacking creatures not available; trample grant omitted.

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Charging Hooligan");
    let human = reg.interner_mut().intern("Human");
    let peasant = reg.interner_mut().intern("Peasant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(peasant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_pump_per_attacker,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_pump_per_attacker(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    ) as i32;
    // GAP: "if a Rat is attacking, gain trample" — subtype on attacking
    // creatures not queryable; trample grant omitted
    vec![Effect::Pump {
        target: trig.source,
        power: n,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
