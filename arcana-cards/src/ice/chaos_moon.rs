//! Chaos Moon — `{3}{R}` enchantment.
//! "At the beginning of each upkeep, count the number of permanents. If
//! the number is odd, until end of turn, red creatures get +1/+1 and
//! whenever a player taps a Mountain for mana, that player adds an
//! additional {R}. If the number is even, until end of turn, red
//! creatures get -1/-1 and if a player taps a Mountain for mana, that
//! Mountain produces colorless mana instead of any other type."
//!
//! The parity is computed at resolution and the matching board-wide
//! pump is applied to all red creatures. Both Mountain mana-replacement
//! clauses are documented GAPs.

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
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chaos Moon");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: parity_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// Count all permanents; odd → red creatures +1/+1 EOT, even → -1/-1 EOT.
fn parity_pump(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the Mountain mana clauses ("adds an additional {R}" /
    // "produces colorless mana instead") are mana-replacement effects
    // with no catalog primitive; only the red-creature pump is modeled.
    let permanents = script::count_matching(
        state,
        &ObjectFilter::permanent(),
        trig.controller,
    );
    let red_creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().with_colors(ColorSet::red()),
        trig.controller,
    );
    let (p, t) = if permanents % 2 == 1 { (1, 1) } else { (-1, -1) };
    vec![Effect::ForEach {
        targets: red_creatures,
        effect: Box::new(Effect::Pump {
            target: arcana_core::objects::NULL_OBJECT_ID,
            power: p,
            toughness: t,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
