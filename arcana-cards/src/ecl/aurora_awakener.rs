//! Aurora Awakener — `{6}{G}` 7/7 Giant Druid. Trample.
//! "Vivid — When this creature enters, reveal cards from the top of your library
//! until you reveal X permanent cards, where X is the number of colors among
//! permanents you control. Put any number of those permanent cards onto the
//! battlefield, then put the rest of the revealed cards on the bottom of your
//! library in a random order."
//!
//! Vivid is not a modeled keyword (no rules of its own — just labels the ETB).
//! The ETB body is inexpressible: RevealUntil finds a single match and places
//! it deterministically; there is no primitive for "reveal until X permanent
//! cards" with a free player choice of how many to put onto the battlefield.
//!
//! GAP: trigger effect — Vivid reveal-until-X-permanents with a player-chosen
//! subset to the battlefield (no matching reveal/put primitive).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::effects::KeywordAbility;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aurora Awakener");
    let giant = reg.interner_mut().intern("Giant");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: vivid_etb,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn vivid_etb(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "reveal until X permanent cards, put any number onto the battlefield,
    // rest on bottom" — no primitive expresses the variable-depth reveal with a
    // free player-chosen subset to the battlefield.
    Vec::new()
}
