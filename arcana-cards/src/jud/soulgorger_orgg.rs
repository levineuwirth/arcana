//! Soulgorger Orgg — `{3}{R}{R}` 6/6 Nightmare Orgg with Trample.
//!
//! Oracle:
//! * Trample.
//! * When this creature enters, you lose all but 1 life.
//! * When this creature leaves the battlefield, you gain life equal to the
//!   life you lost when it entered.
//!
//! Trample is a base characteristic. "Lose all but 1 life" sets your life
//! total to 1 — expressible via `Effect::SetLifeTotal`. The leave trigger
//! gives back "the life you lost when it entered", a value remembered from
//! the earlier ETB; there is no exposed way to recover that historical
//! amount at leave-time, so the gain amount is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Soulgorger Orgg");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let orgg = reg.interner_mut().intern("Orgg");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);
    subtypes.0.insert(orgg);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_lose_all_but_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfLeavesBattlefield,
                intervening_if: None,
                effect: leaves_gain_back,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_lose_all_but_one(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "You lose all but 1 life" — your life total becomes 1.
    vec![Effect::SetLifeTotal { player: trig.controller, amount: 1 }]
}

fn leaves_gain_back(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "gain life equal to the life you lost when it entered" — needs the
    // historical life-lost amount remembered from the earlier ETB; no exposed
    // accessor recovers that value at leave-time.
    Vec::new()
}
