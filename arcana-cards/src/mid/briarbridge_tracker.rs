//! Briarbridge Tracker — `{2}{G}` 2/3 Human Scout with Vigilance.
//!
//! Oracle:
//! * Vigilance — keyword, base characteristic.
//! * "When this creature enters, investigate." — create one Clue token.
//! * "As long as you control a token, this creature gets +2/+0." — GAP: a
//!   static conditional anthem on itself (no trigger word, no cost); not
//!   expressible as a triggered or activated ability.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Briarbridge Tracker");
    let human = reg.interner_mut().intern("Human");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: static "As long as you control a token, this creature gets +2/+0."
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_investigate,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_investigate(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Clue,
        count: 1,
    }]
}
