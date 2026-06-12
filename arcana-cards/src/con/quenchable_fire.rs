//! Quenchable Fire — `{3}{R}` sorcery. "Quenchable Fire deals 3
//! damage to target player or planeswalker. It deals an additional 3
//! damage to that player or planeswalker at the beginning of your
//! next upkeep step unless that player or that planeswalker's
//! controller pays {U} before that step."

use arcana_core::effects::{DelayedWhen, Effect, OptionalPaymentKind};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::PendingTrigger;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quenchable Fire");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Quenchable Fire deals 3 damage to target player or planeswalker. It deals an additional 3 damage to that player or planeswalker at the beginning of your next upkeep step unless that player or that planeswalker's controller pays {U} before that step.".into(),
            target_requirements: vec![TargetRequirement {
                // Best-effort: AnyTarget broader than the printed
                // player-or-planeswalker filter, since no
                // player-or-planeswalker TargetFilter exists.
                filter: TargetFilter::AnyTarget,
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    // The delayed "additional 3 unless {U} is paid" rider, scheduled
    // for the next upkeep. The scheduled slots carry the target: for a
    // player target, `controller` is that player; for a planeswalker
    // target, `source` is the planeswalker's id (its controller is
    // looked up when the trigger fires).
    // GAP (narrow): printed timing is "YOUR next upkeep" and the {U}
    // payment may be made "before that step" — modeled as a pay-{U}
    // choice posted when the next upkeep (whoever's) begins.
    let delayed = match dt {
        DamageTarget::Player(p) => Effect::ScheduleDelayedEffect {
            source: entry.source,
            controller: p,
            when: DelayedWhen::NextUpkeep,
            effect: delayed_burn_player,
        },
        DamageTarget::Object(id) => Effect::ScheduleDelayedEffect {
            source: id,
            controller: entry.controller,
            when: DelayedWhen::NextUpkeep,
            effect: delayed_burn_object,
        },
    };
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: dt,
            amount: 3,
        },
        delayed,
    ]
}

/// Delayed rider, player target: at the next upkeep that player may
/// pay {U}; if they don't, Quenchable Fire deals 3 more damage to
/// them. `pt.controller` is the targeted player.
fn delayed_burn_player(
    _state: &GameState,
    pt: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: pt.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{U}").expect("valid cost")),
        then: Box::new(Effect::Sequence(Vec::new())),
        else_effect: Some(Box::new(Effect::DealDamage {
            source: pt.source,
            target: DamageTarget::Player(pt.controller),
            amount: 3,
        })),
    }]
}

/// Delayed rider, planeswalker target: its current controller may pay
/// {U}; otherwise 3 more damage. `pt.source` is the planeswalker —
/// no-op if it has left the battlefield.
fn delayed_burn_object(
    state: &GameState,
    pt: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(obj) = state.objects.get(pt.source) else { return Vec::new(); };
    vec![Effect::OptionalPayment {
        chooser: obj.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{U}").expect("valid cost")),
        then: Box::new(Effect::Sequence(Vec::new())),
        else_effect: Some(Box::new(Effect::DealDamage {
            // GAP (narrow): the original spell's id isn't carried by the
            // scheduler (the planeswalker occupies the `source` slot), so
            // damage attribution falls back to the planeswalker itself.
            source: pt.source,
            target: DamageTarget::Object(pt.source),
            amount: 3,
        })),
    }]
}
