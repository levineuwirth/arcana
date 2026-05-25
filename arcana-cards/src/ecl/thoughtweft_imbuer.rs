//! Thoughtweft Imbuer — `{3}{W}` 0/5 white Kithkin Advisor. "Whenever a creature you
//! control attacks alone, it gets +X/+X until end of turn, where X is the number of
//! Kithkin you control."
//! GAP: "attacks alone" condition not in engine filter; using CreatureAttacks(You).
//! Kithkin count via script::count_matching + subtype_filter.

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
    let name = reg.interner_mut().intern("Thoughtweft Imbuer");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "attacks alone" not in CreatureAttacks filter
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: on_attacks_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attacks_pump(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &script::subtype_filter(reg, "Kithkin").controlled_by(ControllerConstraint::You),
        trig.controller,
    ) as i32;
    if n == 0 {
        return Vec::new();
    }
    // GAP: we pump trig.source (self) instead of the attacking creature; no
    // "triggering attacker id" accessor available
    vec![Effect::Pump {
        target: trig.source,
        power: n,
        toughness: n,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
