//! Thunderbreak Regent — `{2}{R}{R}` 4/4 Dragon.
//! - Flying.
//! - Whenever a Dragon you control becomes the target of a spell or
//!   ability an opponent controls, this creature deals 3 damage to that
//!   player.
//!
//! Partial: only `SelfBecomesTarget` exists (no "a Dragon you control
//! becomes the target" filtered variant). This faithfully covers the
//! common case where THIS Dragon is the targeted one; other Dragons you
//! control are GAP'd. "That player" is resolved as the single opponent
//! (the targeting source is opponent-controlled).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thunderbreak Regent");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: oracle is "a Dragon YOU CONTROL becomes the target";
                // only SelfBecomesTarget exists, so this covers this Dragon.
                trigger_condition: TriggerCondition::SelfBecomesTarget {
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: bolt_that_player,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// This creature deals 3 damage to that player (the opponent who
/// targeted a Dragon you control).
fn bolt_that_player(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opps = script::opponents(state, trig.controller);
    let Some(p) = opps.first() else { return Vec::new(); };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(*p),
        amount: 3,
    }]
}
