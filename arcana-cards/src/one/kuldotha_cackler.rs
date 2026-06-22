//! Kuldotha Cackler — `{2}{R}` 2/3 Creature — Phyrexian Hyena.
//! Trample.
//! Whenever this creature attacks, it gets +X/+0 until end of turn, where X is
//! the number of permanents you control with oil counters on them.

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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kuldotha Cackler");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let hyena = reg.interner_mut().intern("Hyena");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(hyena);
    // Pre-intern the "oil" counter name so the resolver can recover it.
    let _oil = reg.interner_mut().intern("oil");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_pump,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_pump(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let oil = match reg.interner().lookup("oil").map(CounterKind::Named) {
        Some(k) => k,
        None => return Vec::new(),
    };
    let filter = ObjectFilter {
        has_counter: Some(oil),
        ..ObjectFilter::new().controlled_by(ControllerConstraint::You)
    };
    let x = script::count_matching(state, &filter, trig.controller) as i32;
    vec![Effect::Pump {
        target: trig.source,
        power: x,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
