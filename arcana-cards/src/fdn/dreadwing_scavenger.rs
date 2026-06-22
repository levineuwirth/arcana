//! Dreadwing Scavenger — `{1}{U}{B}` 2/2 Creature — Nightmare Bird.
//!
//! Oracle:
//! * Flying.
//! * Whenever this creature enters or attacks, draw a card, then discard a
//!   card.
//! * Threshold — This creature gets +1/+1 and has deathtouch as long as there
//!   are seven or more cards in your graveyard.
//!
//! Decomposition: Flying keyword + the "enters or attacks" clause as TWO
//! triggers (no combined enters-or-attacks condition) both running the same
//! draw-then-discard resolver.
//!
//! GAP: the Threshold static (+1/+1 and deathtouch while 7+ cards in
//! graveyard) is a continuous self-buff conditional on graveyard size — not
//! expressible as a triggered/activated ability; omitted.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dreadwing Scavenger");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);
    subtypes.0.insert(bird);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn loot(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
