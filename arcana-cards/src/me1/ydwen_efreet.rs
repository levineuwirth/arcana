//! Ydwen Efreet — `{R}{R}{R}` 3/6 red Creature — Efreet. "Whenever this
//! creature blocks, flip a coin. If you lose the flip, remove this
//! creature from combat and it can't block this turn. Creatures it was
//! blocking that had become blocked by only this creature this combat
//! become unblocked."
//!
//! On a lost coin flip we forbid this creature from blocking for the
//! turn. "Remove this creature from combat" and "creatures it was
//! blocking become unblocked" have no catalog effect, so those riders
//! are GAP material.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Ydwen Efreet");
    let efreet = reg.interner_mut().intern("Efreet");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(efreet);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBlocks,
                intervening_if: None,
                effect: on_blocks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_blocks(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "Flip a coin. If you lose the flip, [it can't block this turn]."
    // GAP: "remove this creature from combat" and "creatures it was
    // blocking that had become blocked by only this creature become
    // unblocked" have no catalog effect; only the can't-block rider is
    // expressed (on a lost flip).
    vec![Effect::FlipCoin {
        player: trig.controller,
        win: Box::new(Effect::Sequence(vec![])),
        lose: Some(Box::new(Effect::ForbidBlocking {
            target: trig.source,
            duration: Duration::EndOfTurn,
        })),
    }]
}
