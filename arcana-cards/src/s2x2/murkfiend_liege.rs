//! Murkfiend Liege — `{2}{G/U}{G/U}{G/U}` 4/4 Creature — Horror.
//!
//! Oracle:
//! * Other green creatures you control get +1/+1. (static — GAP)
//! * Other blue creatures you control get +1/+1. (static — GAP)
//! * Untap all green and/or blue creatures you control during each other
//!   player's untap step.
//!
//! GAP: the two anthem statics ("Other green/blue creatures you control get
//! +1/+1") are continuous static abilities with no triggered/activated hook.
//!
//! The untap line is modeled as a triggered ability at the beginning of each
//! opponent's untap step (a faithful approximation of the "during the untap
//! step" continuous untap).

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Murkfiend Liege");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G/U}{G/U}{G/U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Untap,
                whose: ControllerConstraint::Opponent,
            },
            intervening_if: None,
            effect: untap_green_blue,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn untap_green_blue(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "green and/or blue" is a color OR; `with_colors` requires ALL listed
    // colors, so query green and blue separately and union the ids.
    let green = ObjectFilter::creature()
        .with_colors(ColorSet::green())
        .controlled_by(ControllerConstraint::You);
    let blue = ObjectFilter::creature()
        .with_colors(ColorSet::blue())
        .controlled_by(ControllerConstraint::You);
    let mut ids = script::ids_matching(state, &green, trig.controller);
    for id in script::ids_matching(state, &blue, trig.controller) {
        if !ids.contains(&id) {
            ids.push(id);
        }
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Untap {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}
