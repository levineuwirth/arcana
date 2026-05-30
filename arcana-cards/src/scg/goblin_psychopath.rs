//! Goblin Psychopath — `{3}{R}` 5/5 Goblin Mutant. "Whenever this creature attacks or
//! blocks, flip a coin. If you lose the flip, the next time it would deal combat damage
//! this turn, it deals that damage to you instead."
//! Trigger: SelfBlocksOrBecomesBlocked combined with SelfAttacks — closest match is
//! SelfBlocksOrBecomesBlocked for blocks, but "attacks or blocks" needs two triggers;
//! GAP: no combined "attacks or blocks" TriggerCondition variant.
//! GAP: "next time it would deal combat damage, deal to you instead" — damage redirect
//! replacement effect not in Effect catalog.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Goblin Psychopath");
    let goblin = reg.interner_mut().intern("Goblin");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(mutant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: trigger — "attacks or blocks" is two separate conditions; SelfAttacks
            // covers the attacks half; a second trigger for SelfBlocksOrBecomesBlocked
            // would cover the blocks half. Using SelfAttacks here as the primary trigger;
            // the blocks half is unregistered.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_or_blocks_flip,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfBlocksOrBecomesBlocked,
                intervening_if: None,
                effect: attacks_or_blocks_flip,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attacks_or_blocks_flip(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if you lose the flip, the next time it would deal combat damage this turn,
    // it deals that damage to you instead" — damage redirect replacement effect is not
    // in the Effect catalog. Emitting the coin flip with a no-op win branch and a no-op
    // lose branch since the redirect cannot be expressed.
    vec![Effect::FlipCoin {
        player: trig.controller,
        win: Box::new(Effect::Sequence(vec![])),
        // GAP: on loss, redirect next combat damage from this creature to its controller —
        // no Effect::RedirectDamage or replacement-effect variant available.
        lose: Some(Box::new(Effect::Sequence(vec![]))),
    }]
}
