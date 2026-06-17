//! Innocuous Researcher — `{3}{G}` 3/4 Centaur Detective (green).
//! "Parley — Whenever this creature attacks, each player reveals the top card
//!  of their library. For each nonland card revealed this way, you
//!  investigate. Then each player draws a card.
//!  At the beginning of your end step, you may untap all lands you control.
//!  If you do, you can't cast spells until your next turn."
//!
//! Investigate / Parley are not supported keywords → keywords empty.
//! - Attack trigger: the reveal-tops + count-nonlands + investigate body is
//!   not expressible (no reveal-and-count primitive; the per-nonland Clue
//!   count is resolution-time) → that portion is GAP'd. The "then each player
//!   draws a card" tail IS wired (Sequence of DrawCards over all players).
//! - End-step ability: the optional untap-all-lands-with-a-cast-restriction
//!   downside is not expressible → GAP'd entirely.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Innocuous Researcher");
    let centaur = reg.interner_mut().intern("Centaur");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: parley_each_player_draws,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
        // GAP: "At the beginning of your end step, you may untap all lands ...
        // you can't cast spells until your next turn." — optional untap with a
        // self-imposed casting restriction is not expressible.
    )
}

fn parley_each_player_draws(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the reveal-top-of-library + "for each nonland card revealed, you
    // investigate" portion is not expressible (no reveal-and-count primitive).
    // Only the "then each player draws a card" tail is emitted.
    let players = script::all_players(state);
    let mut effects = Vec::new();
    for p in players {
        effects.push(Effect::DrawCards {
            player: p,
            count: 1,
        });
    }
    vec![Effect::Sequence(effects)]
}
