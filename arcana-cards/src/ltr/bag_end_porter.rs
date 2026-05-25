//! Bag End Porter — `{3}{G}` 4/4 green Creature — Dwarf.
//! "Whenever this creature attacks, it gets +X/+X until end of turn, where X
//! is the number of legendary creatures you control."

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
    let name = reg.interner_mut().intern("Bag End Porter");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack_legendary_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attack_legendary_pump(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: ObjectFilter has no with_supertypes; using creature filter with
    // You constraint as approximation; X will count all your creatures
    // instead of only legendary ones.
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You);
    let x = script::count_matching(state, &filter, trig.controller) as i32;
    vec![Effect::Pump {
        target: trig.source,
        power: x,
        toughness: x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
