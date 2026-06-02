//! Two-Headed Giant — `{2}{R}{R}` 4/4 red Giant Warrior. "Whenever
//! this creature attacks, flip two coins. If both coins come up heads,
//! this creature gains double strike until end of turn. If both coins
//! come up tails, this creature gains menace until end of turn."
//!
//! Modeled with nested `FlipCoin`: the first coin selects a branch and
//! the second coin must agree (both heads → double strike, both tails →
//! menace); any split result does nothing, matching the oracle.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Two-Headed Giant");
    let giant = reg.interner_mut().intern("Giant");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: flip_two_coins,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn flip_two_coins(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let double_strike = Effect::GrantKeyword {
        target: trig.source,
        keyword: KeywordAbility::DoubleStrike,
        duration: Duration::EndOfTurn,
    };
    let menace = Effect::GrantKeyword {
        target: trig.source,
        keyword: KeywordAbility::Menace,
        duration: Duration::EndOfTurn,
    };
    // First coin heads -> second coin must also be heads for double strike;
    // first coin tails -> second coin must also be tails for menace.
    vec![Effect::FlipCoin {
        player: trig.controller,
        win: Box::new(Effect::FlipCoin {
            player: trig.controller,
            win: Box::new(double_strike),
            lose: None,
        }),
        lose: Some(Box::new(Effect::FlipCoin {
            player: trig.controller,
            win: Box::new(Effect::Sequence(vec![])),
            lose: Some(Box::new(menace)),
        })),
    }]
}
