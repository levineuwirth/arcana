//! Raven Familiar — `{2}{U}` 1/2 Bird with Flying.
//! Flying.
//! Echo {2}{U}.
//! When this creature enters, look at the top three cards of your library. Put
//! one of them into your hand and the rest on the bottom of your library in any
//! order.
//!
//! Flying is a base characteristic. The ETB dig is `Effect::DigTopN` (look at
//! top 3, take one to hand, rest to bottom). Echo is not part of the usable
//! keyword surface, so the upkeep "pay or sacrifice" mechanic is GAP'd.

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Raven Familiar");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);

    // GAP: Echo {2}{U} — not part of the usable keyword surface; the upkeep
    // "pay echo cost or sacrifice" mechanic is unmodeled.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_dig_three,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_dig_three(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 3,
        filter: None,
        rest: DigRest::BottomRandom,
    }]
}
