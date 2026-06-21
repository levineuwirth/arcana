//! Boomer Scrapper — `{1}{B}{R}` 1/1 Human Soldier.
//! Whenever this creature enters or attacks, you lose 1 life and create a
//! Junk token.
//! Whenever a token you control leaves the battlefield, put a +1/+1 counter
//! on this creature.
//!
//! The "enters or attacks" clause is decomposed into two triggers (an
//! enters trigger + an attacks trigger), each losing 1 life and minting a
//! Junk token. There is no `CommodityToken::Junk`, so the Junk token is
//! minted as a plain artifact token; its printed "{T}, Sacrifice: exile the
//! top card, you may play it" activated ability is GAP'd (token activated
//! abilities aren't dispatched from a hand-rolled TokenDefinition).
//! The "token you control leaves the battlefield" trigger is wired
//! (tokens-only, controlled-by-you, battlefield -> graveyard per the
//! catalog leaves-the-battlefield convention).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boomer Scrapper");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let _junk = reg.interner_mut().intern("Junk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // Whenever this creature enters ... you lose 1 life and create a Junk token.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: lose_life_make_junk,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // ... or attacks, you lose 1 life and create a Junk token.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: lose_life_make_junk,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Whenever a token you control leaves the battlefield, put a +1/+1 counter on this creature.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .controlled_by(ControllerConstraint::You)
                        .tokens_only(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: counter_on_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn lose_life_make_junk(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let junk = reg.interner().lookup("Junk").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(junk);
    vec![
        Effect::LoseLife {
            player: trig.controller,
            amount: 1,
        },
        // GAP: the Junk token's printed activated ability ("{T}, Sacrifice
        // this token: Exile the top card of your library; you may play it
        // this turn") is not wired on a hand-rolled token.
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: junk,
                colors: ColorSet::colorless(),
                types: TypeLine::ARTIFACT.into(),
                subtypes,
                power: None,
                toughness: None,
                keywords: vec![],
                abilities: vec![],
            },
        },
    ]
}

fn counter_on_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
