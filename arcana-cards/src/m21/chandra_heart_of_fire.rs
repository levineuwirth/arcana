//! Chandra, Heart of Fire — `{3}{R}{R}` Legendary Planeswalker — Chandra, loyalty 5.
//!
//! +1: Discard your hand, then exile the top three cards of your library. Until
//!   end of turn, you may play cards exiled this way.
//! +1: Chandra deals 2 damage to any target.
//! −9: Search your graveyard and library for any number of red instant and/or
//!   sorcery cards, exile them, then shuffle. You may cast them this turn. Add
//!   six {R}.
//!
//! # Scope
//! GAP: the −9 ability (search BOTH graveyard and library for any number of red
//!   instant/sorcery, exile, cast-this-turn, add six {R}) needs multi-zone
//!   any-number search + cast-from-exile + mana — not expressible as one effect.
//!   Ability shell declared with its −9 cost, effect empty.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Heart of Fire");
    let chandra = reg.interner_mut().intern("Chandra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chandra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Discard your hand, then exile the top three cards of your library. Until end of turn, you may play cards exiled this way.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_wheel,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Chandra deals 2 damage to any target.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_damage,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-9: Search your graveyard and library for any number of red instant and/or sorcery cards, exile them, then shuffle. You may cast them this turn. Add six {R}.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 9)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_nine_gap,
            }),
    )
}

fn plus_one_wheel(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let hand = script::hand_size(state, ctx.controller);
    let mut effects = Vec::new();
    if hand > 0 {
        effects.push(Effect::Discard {
            player: ctx.controller,
            count: hand,
            choice: DiscardChoice::ControllerChooses,
        });
    }
    effects.push(Effect::ImpulseExile { player: ctx.controller, count: 3 });
    effects
}

fn plus_one_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage { source: ctx.source, target: dt, amount: 2 }]
}

fn minus_nine_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: multi-zone any-number search + cast-from-exile + add six {R}.
    Vec::new()
}
