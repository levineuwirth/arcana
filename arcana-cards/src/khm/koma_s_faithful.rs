//! Koma's Faithful — `{2}{B}` 3/1 Elf Cleric with Lifelink.
//!
//! Oracle:
//! * Lifelink
//! * When this creature dies, each player mills three cards.
//!
//! Lifelink is a base keyword. (Scryfall lists "Mill" as a keyword, but it
//! is the dies-trigger effect below, not a creature keyword ability — so
//! the keywords vec holds only Lifelink.) The dies trigger mills three
//! cards for each player, built as one Mill per player wrapped in a
//! Sequence.

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
    let name = reg.interner_mut().intern("Koma's Faithful");
    let elf = reg.interner_mut().intern("Elf");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_mill_each,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dies_mill_each(state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mills: Vec<Effect> = script::all_players(state)
        .into_iter()
        .map(|p| Effect::Mill { player: p, count: 3 })
        .collect();
    vec![Effect::Sequence(mills)]
}
