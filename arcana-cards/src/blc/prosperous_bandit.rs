//! Prosperous Bandit — `{2}{R}` 2/2 Raccoon Rogue.
//!
//! * Offspring {1}, First strike. (Scryfall "Treasure" is a token tag,
//!   not a keyword.)
//! * Whenever this creature deals combat damage to a player, create that
//!   many tapped Treasure tokens. (DamageDealt to a player, combat only;
//!   the count is the combat damage dealt via `trig.damage_amount()`. The
//!   "tapped" rider is a minor fidelity gap — commodity tokens have no
//!   tapped option.)

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::state::GameState;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Prosperous Bandit");
    let raccoon = reg.interner_mut().intern("Raccoon");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(raccoon);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![
            KeywordAbility::Offspring(ManaCost::parse("{1}").expect("valid cost")),
            KeywordAbility::FirstStrike,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: make_treasures,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Combat damage to a player: create that many tapped Treasure tokens.
fn make_treasures(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: n,
    }]
}
