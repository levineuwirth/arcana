//! Kaima, the Fractured Calm — `{2}{R}{G}` 3/3 red-green Legendary Creature — Spirit.
//! "At the beginning of your end step, goad each creature your opponents control that's
//! enchanted by an Aura you control. Put a +1/+1 counter on Kaima for each creature
//! goaded this way."
//! GAP: "each creature enchanted by an Aura you control" filter not in ObjectFilter.

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
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaima, the Fractured Calm");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_goad,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_goad(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: filter "enchanted by an Aura you control" not in ObjectFilter;
    // goading all opponent creatures as approximation.
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
        trig.controller,
    );
    let goaded_count = ids.len() as u32;
    let mut effects: Vec<Effect> = ids.into_iter().map(|id| Effect::Goad {
        target: id,
        goader: trig.controller,
        duration: Duration::EndOfTurn,
    }).collect();
    if goaded_count > 0 {
        effects.push(Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: goaded_count,
        });
    }
    effects
}
