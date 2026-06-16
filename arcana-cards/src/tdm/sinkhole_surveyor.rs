//! Sinkhole Surveyor — `{1}{B}` 1/3 Bird Scout with Flying.
//! "Whenever this creature attacks, you lose 1 life and this creature
//! endures 1. (Put a +1/+1 counter on it or create a 1/1 white Spirit
//! creature token.)"
//!
//! Endure is not an engine keyword/effect (a player-choice modal between
//! a counter and a token) — only the life-loss half of the attack
//! trigger is expressed; the endure is GAP'd. Scryfall also lists
//! "Endure" as a keyword, which has no KeywordAbility variant.

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
    let name = reg.interner_mut().intern("Sinkhole Surveyor");
    let bird = reg.interner_mut().intern("Bird");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: "Endure" keyword has no KeywordAbility variant.
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: on_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_attack(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "endures 1" — player-choice modal (counter OR 1/1 Spirit token); no Endure effect/modal here.
    vec![Effect::LoseLife { player: trig.controller, amount: 1 }]
}
