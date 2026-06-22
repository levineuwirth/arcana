//! Storm Fleet Negotiator — `{2}{U}` 2/2 Siren Pirate with Flying.
//!
//! Oracle:
//! * Flying
//! * "Parley — Whenever this creature attacks, each player reveals the top
//!   card of their library. For each nonland card revealed this way, you
//!   create a Map token. Then each player draws a card."
//!
//! GAP: "each player reveals the top card; for each nonland card revealed, you
//! create a Map token" — there is no primitive to reveal each player's library
//! top and count the nonland reveals, so the Map-token creation is omitted.
//! The "then each player draws a card" clause IS expressible and is emitted.
//! (Explore / Parley are ability words, not usable KeywordAbility variants.)

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Storm Fleet Negotiator");
    let siren = reg.interner_mut().intern("Siren");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siren);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
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
    )
}

fn parley_each_player_draws(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reveal each player's library top + create a Map token per nonland
    // revealed — no reveal-and-count primitive. Emit only "each player draws a
    // card", which is fully expressible.
    let effects: Vec<Effect> = script::all_players(state)
        .into_iter()
        .map(|p| Effect::DrawCards { player: p, count: 1 })
        .collect();
    vec![Effect::Sequence(effects)]
}
