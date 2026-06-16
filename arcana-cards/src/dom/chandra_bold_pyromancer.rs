//! Chandra, Bold Pyromancer — `{4}{R}{R}` Legendary Planeswalker —
//! Chandra, starting loyalty 4. Red.
//!
//! Oracle text:
//! * `+1`: Add {R}{R}. Chandra deals 2 damage to target player.
//! * `−3`: Chandra deals 3 damage to target creature or planeswalker.
//! * `−7`: Chandra deals 10 damage to target player and each creature
//!   and planeswalker they control.
//!
//! # Scope
//!
//! * `+1` (add {R}{R} + 2 damage to target player) is expressible.
//! * `−3` (3 damage to a creature-or-planeswalker target) is
//!   expressible via a `Permanent` filter for creature/planeswalker.
//! * `−7` deals 10 to a player AND sweeps each creature/planeswalker
//!   that player controls — a board sweep keyed off the chosen player —
//!   not expressible as a single damage effect; the targeted-player
//!   portion alone would be unfaithful, so GAP'd.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Bold Pyromancer");
    let chandra = reg.interner_mut().intern("Chandra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chandra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Add {R}{R}. Chandra deals 2 damage to target \
                       player.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Chandra deals 3 damage to target creature or \
                       planeswalker.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::permanent()
                        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER))),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Chandra deals 10 damage to target player and \
                       each creature and planeswalker they control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven,
            }),
    )
}

/// `+1`: add {R}{R}, then 2 damage to target player.
fn plus_one(_s: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![
                ManaUnit::plain(ManaColor::Red, ctx.source),
                ManaUnit::plain(ManaColor::Red, ctx.source),
            ],
        },
        Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(*p),
            amount: 2,
        },
    ]
}

/// `−3`: 3 damage to target creature or planeswalker.
fn minus_three(_s: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: 3,
    }]
}

/// `−7`: 10 damage to target player and each creature/planeswalker they
/// control.
fn minus_seven(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: damage to the targeted player PLUS a board sweep of every
    // creature and planeswalker that player controls is not expressible
    // as a single damage effect; emitting only the player half would be
    // unfaithful.
    Vec::new()
}
