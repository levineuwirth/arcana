//! Woebringer Demon — `{3}{B}{B}` 4/4 Creature — Demon.
//!
//! * Flying — evergreen keyword.
//! * At the beginning of each player's upkeep, that player sacrifices a
//!   creature of their choice. If the player can't, sacrifice this creature. —
//!   `StepBegins { Upkeep, Any }`; the upkeep belongs to the active player, so
//!   the effect reads `state.active_player()` and that player sacrifices one
//!   creature.
//!   GAP: the "If the player can't, sacrifice this creature" fallback — no
//!   "if a sacrifice could not be made, do X instead" conditional primitive
//!   here; the main sacrifice is wired faithfully.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::effects::KeywordAbility;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Woebringer Demon");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: each_upkeep_sacrifice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "that player sacrifices a creature of their choice."
/// GAP: "If the player can't, sacrifice this creature" fallback is unmodeled.
fn each_upkeep_sacrifice(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let them = state.active_player();
    vec![Effect::Sacrifice {
        player: them,
        filter: ObjectFilter::creature(),
        count: 1,
    }]
}
