//! Shoreline Looter — `{1}{U}` 1/1 Rat Rogue.
//! "This creature can't be blocked.
//!  Threshold — Whenever this creature deals combat damage to a player,
//!  draw a card. Then discard a card unless there are seven or more cards
//!  in your graveyard."
//!
//! The unblockable static is modeled as a self-targeting CantBeBlocked
//! while on the battlefield. The combat-damage trigger draws, then the
//! discard is gated by an intervening-if-style graveyard threshold —
//! expressed at resolution via a conditional on graveyard size.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shoreline Looter");
    let rat = reg.interner_mut().intern("Rat");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "This creature can't be blocked." (static, modeled by
            // an ETB self-trigger that grants the unblockable status
            // while on the battlefield).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_unblockable,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Threshold combat-damage trigger.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: arcana_core::targets::ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: draw_then_maybe_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_unblockable(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CantBeBlocked {
        target: trig.source,
        duration: Duration::WhileSourceOnBattlefield,
    }]
}

fn draw_then_maybe_discard(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let gy = script::graveyard_size(state, trig.controller);
    let mut effects = vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }];
    // "Then discard a card unless there are seven or more cards in your
    // graveyard." (Graveyard size is measured before the draw; after a
    // draw the threshold check is the same total. Seven-or-more = skip.)
    if gy < 7 {
        effects.push(Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        });
    }
    effects
}
