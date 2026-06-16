//! Steel-Plume Marshal — `{3}{W}{W}` 3/3 Bird Soldier.
//! Flying.
//! "Whenever this creature attacks, other attacking creatures you control with
//! flying get +2/+2 until end of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
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
    let name = reg.interner_mut().intern("Steel-Plume Marshal");
    let bird = reg.interner_mut().intern("Bird");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: pump_flying_attackers,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn pump_flying_attackers(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_keyword(KeywordAbility::Flying)
        .attacking_only();
    let ids: Vec<_> = script::ids_matching(state, &filter, trig.controller)
        .into_iter()
        .filter(|&id| id != trig.source)
        .collect();
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: 2,
            toughness: 2,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
