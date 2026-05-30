//! Goblin Traprunner — `{3}{R}` 4/2 red Creature — Goblin.
//! "Whenever this creature attacks, flip three coins. For each flip you win,
//! create a 1/1 red Goblin creature token that's tapped and attacking."
//! GAP: tokens created by this effect are not marked tapped-and-attacking
//! (the token entering tapped + attacking state is not modeled in CreateToken).
//! The three-coin flip is expressed as three sequential FlipCoin effects.

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Goblin Traprunner");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    // Pre-intern for resolver
    let _goblin_tok = reg.interner_mut().intern("Goblin");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: flip_three_coins,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_goblin_token(controller: arcana_core::types::PlayerId, reg: &CardRegistry) -> Effect {
    let goblin = reg.interner().lookup("Goblin")
        .expect("Goblin interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    Effect::CreateToken {
        controller,
        token: TokenDefinition {
            name: goblin,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }
}

fn flip_three_coins(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let p = trig.controller;
    // Three independent coin flips; each win creates a Goblin token.
    let coin3 = Effect::FlipCoin {
        player: p,
        win: Box::new(make_goblin_token(p, reg)),
        lose: None,
    };
    let coin2 = Effect::FlipCoin {
        player: p,
        win: Box::new(make_goblin_token(p, reg)),
        lose: None,
    };
    let coin1 = Effect::FlipCoin {
        player: p,
        win: Box::new(make_goblin_token(p, reg)),
        lose: None,
    };
    vec![Effect::Sequence(vec![coin1, coin2, coin3])]
}
