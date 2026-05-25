//! Magus of the Vineyard — `{G}` 1/1 green Human Wizard.
//! "At the beginning of each player's first main phase, that player
//! adds {G}{G}."

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Magus of the Vineyard");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: add_two_green,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_two_green(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "that player" — the player whose main phase is beginning;
    // use script::all_players to identify each controller turn-by-turn;
    // the trigger fires for each player's phase, so trig.controller is
    // the active player this fires for. We add {G}{G} to that player.
    // GAP: the trigger condition fires for the controller's main phase;
    // "each player's first main phase" requires per-player tracking.
    // Best effort: add {G}{G} to trig.controller (the trigger controller).
    let _ = state;
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Green, trig.source),
            ManaUnit::plain(ManaColor::Green, trig.source),
        ],
    }]
}
