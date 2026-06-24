//! Elder Spawn — `{4}{U}{U}{U}` 6/6 Spawn.
//! "At the beginning of your upkeep, unless you sacrifice an Island,
//!  sacrifice this creature and it deals 6 damage to you." — GAP: the
//!  payment is "sacrifice an ISLAND" (a subtype-specific sacrifice).
//!  SacrificeFilter only has Land (any land), not a Land-subtype filter, so
//!  using it would over-include non-Island lands — a meaningful semantic
//!  change. Subtype-specific sacrifice payment isn't expressible; GAP'd.
//! "This creature can't be blocked by red creatures." — GAP: source-filtered
//!  blocking restriction (can't-be-blocked-by-<color>) is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elder Spawn");
    let spawn = reg.interner_mut().intern("Spawn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spawn);

    // GAP: "This creature can't be blocked by red creatures." — source-color-
    // filtered blocking restriction is not expressible.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_unless_sacrifice_island,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_unless_sacrifice_island(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "unless you sacrifice an ISLAND, sacrifice this creature and it
    // deals 6 damage to you." — the payment is a subtype-specific sacrifice
    // (an Island), but SacrificeFilter only offers Land (any land); using it
    // would over-include non-Island lands. Subtype-specific sacrifice payment
    // isn't expressible, so the gate (and its conditional punishment) is GAP'd.
    Vec::new()
}
