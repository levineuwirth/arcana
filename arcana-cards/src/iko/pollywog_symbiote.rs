//! Pollywog Symbiote — `{1}{U}` 1/3 Frog.
//! "Each creature spell you cast costs {1} less to cast if it has
//! mutate."
//! "Whenever you cast a creature spell, if it has mutate, draw a card,
//! then discard a card."
//!
//! The cost-reduction static is not expressible (no cost-modification
//! Effect), so it is GAP'd. The cast trigger fires on casting any
//! creature spell and draws-then-discards; the "if it has mutate"
//! intervening-if has no predicate (mutate isn't exposed), so the gate
//! is GAP'd to None.

use arcana_core::effects::{DiscardChoice, Effect};
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

// GAP (static): "Each creature spell you cast costs {1} less if it has mutate"
// — no cost-modification Effect is available.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pollywog Symbiote");
    let frog = reg.interner_mut().intern("Frog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP (intervening-if): "if it has mutate" has no predicate; gate dropped.
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter::new().with_types(TypeLine::CREATURE.into())),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: draw_then_discard,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn draw_then_discard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
