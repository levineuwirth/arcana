//! Underworld Dreams — `{B}{B}{B}` enchantment (Legends, 1994).
//! "Whenever an opponent draws a card, Underworld Dreams deals 1
//! damage to them." The seed touchstone for the TriggeredEnchantment
//! card-gen shape: a non-Aura enchantment whose entire game text is
//! one triggered ability.
//!
//! # Rules references
//!
//! * CR 603.1 — triggered abilities function from the battlefield by
//!   default; the enchantment carries a [`TriggeredAbilityDef`]
//!   exactly like a creature would (Elvish Visionary class) — only
//!   the type line differs.
//! * CR 120.3 — each draw is a separate event, so drawing three
//!   cards fires this three times ([`TriggerFrequency::EachTime`]).
//!
//! # Implementation notes
//!
//! The drawing player is read off the triggering
//! [`GameEvent::DrawCard`] event, which is multiplayer-correct
//! ("them" is whoever drew, not "the opponent" of a 2-player game).
//! This is a hand-written seed; generated cards should prefer the
//! typed `trig.*` accessors where one exists.

use arcana_core::effects::Effect;
use arcana_core::events::{DamageTarget, GameEvent};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Underworld Dreams");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: ping_drawer,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…deals 1 damage to them" — the player who drew, read off the
/// triggering DrawCard event (the condition already filtered to
/// opponents-only).
fn ping_drawer(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let GameEvent::DrawCard { player, .. } = trig.trigger_event else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        target: DamageTarget::Player(player),
        amount: 1,
        source: trig.source,
    }]
}
