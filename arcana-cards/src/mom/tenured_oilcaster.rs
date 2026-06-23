//! Tenured Oilcaster — `{3}{B}` 2/4 Phyrexian Wizard with Menace.
//!
//! Oracle:
//! * Menace
//! * "This creature gets +3/+0 as long as an opponent has eight or more cards
//!   in their graveyard." (static — GAP'd)
//! * "Whenever this creature attacks or blocks, each player mills a card."
//!
//! Menace is a base keyword. The conditional static pump is not expressible
//! and is GAP'd. The "attacks or blocks" mill is decomposed into two triggers
//! — one on SelfAttacks and one on SelfBlocks — each milling every player one
//! card. (Scryfall lists "Mill" as a keyword for this card, but that is the
//! mill ABILITY's keyword tag, not a standalone keyword ability — it is
//! realized as the trigger, not a `KeywordAbility` variant.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tenured Oilcaster");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(wizard);

    // GAP static: "This creature gets +3/+0 as long as an opponent has eight
    // or more cards in their graveyard." — no conditional continuous static
    // self-pump primitive in the demonstrated API.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: each_player_mills,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfBlocks,
                intervening_if: None,
                effect: each_player_mills,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn each_player_mills(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    script::all_players(state)
        .into_iter()
        .map(|p| Effect::Mill { player: p, count: 1 })
        .collect()
}
