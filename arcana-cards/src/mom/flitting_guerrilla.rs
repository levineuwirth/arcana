//! Flitting Guerrilla — `{2}{B}` 2/2 Creature — Faerie Rogue with Flying.
//!
//! Oracle:
//! * "Flying" — base keyword.
//! * "When this creature dies, each player mills two cards. Then you may exile
//!   this card. When you do, put target creature or battle card from your
//!   graveyard on top of your library."
//!
//! Modeled: the dies trigger mills two cards for each player (one
//! `Effect::Mill` per player, wrapped in a `Sequence`). GAP'd: the reflexive
//! "Then you may exile this card. When you do, put target … on top of your
//! library" chain — an optional self-exile that posts a reflexive "when you
//! do" sub-trigger with its own target; there is no reflexive-trigger primitive
//! in the demonstrated surface.
//! (The Scryfall "Mill" keyword is reminder/rules text, not a KeywordAbility.)

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
    let name = reg.interner_mut().intern("Flitting Guerrilla");
    let faerie = reg.interner_mut().intern("Faerie");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
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
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: dies_each_player_mills,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dies_each_player_mills(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "each player mills two cards"
    let mills: Vec<Effect> = script::all_players(state)
        .into_iter()
        .map(|p| Effect::Mill { player: p, count: 2 })
        .collect();
    // GAP: reflexive "Then you may exile this card. When you do, put target
    // creature or battle card from your graveyard on top of your library."
    vec![Effect::Sequence(mills)]
}
