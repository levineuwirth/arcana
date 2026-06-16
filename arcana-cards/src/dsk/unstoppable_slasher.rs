//! Unstoppable Slasher — `{2}{B}` 2/3 Zombie Assassin with Deathtouch.
//! "Whenever this creature deals combat damage to a player, they lose half
//! their life, rounded up."
//! "When this creature dies, if it had no counters on it, return it to the
//! battlefield tapped under its owner's control with two stun counters on
//! it."
//!
//! The combat-damage source can't be restricted to this creature
//! specifically (no self-source filter), so it watches your creatures'
//! combat damage to a player — over-firing vs. the printed gate. The dies
//! trigger's self-recursion (return the now-dead card tapped with two stun
//! counters, gated on having no counters) is not expressible, so its body
//! is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unstoppable Slasher");
    let zombie = reg.interner_mut().intern("Zombie");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: source can't be restricted to this creature only.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: lose_half_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_return,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn lose_half_life(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(p) = trig.damaged_player() else {
        return Vec::new();
    };
    let life = script::life(state, p).max(0) as u32;
    let half_round_up = (life + 1) / 2;
    vec![Effect::LoseLife {
        player: p,
        amount: half_round_up,
    }]
}

fn dies_return(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "if it had no counters on it, return it to the battlefield tapped
    // with two stun counters" — self-recursion on a now-dead object with a
    // counter gate, no expressible primitive.
    Vec::new()
}
