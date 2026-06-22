//! Loxodon Peacekeeper — `{1}{W}` 4/4 Creature — Elephant Soldier.
//!
//! Oracle:
//! * At the beginning of your upkeep, the player with the lowest life total
//!   gains control of this creature. If two or more players are tied for
//!   lowest life total, you choose one of them, and that player gains control
//!   of this creature. (Wired: at upkeep, compute the lowest-life player from
//!   all players and ChangeControl this creature to them. FIDELITY GAP: ties
//!   are broken deterministically — the first player with the minimum life is
//!   chosen rather than prompting the controller.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Loxodon Peacekeeper");
    let elephant = reg.interner_mut().intern("Elephant");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: lowest_life_gains_control,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn lowest_life_gains_control(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let players = script::all_players(state);
    let Some(lowest) = players
        .iter()
        .min_by_key(|&&p| script::life(state, p))
        .copied()
    else {
        return Vec::new();
    };
    vec![Effect::ChangeControl {
        target: trig.source,
        new_controller: lowest,
    }]
}
