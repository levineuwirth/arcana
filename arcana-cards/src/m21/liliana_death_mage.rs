//! Liliana, Death Mage — `{4}{B}{B}` legendary planeswalker, starting loyalty 5.
//!
//! Loyalty abilities (CR 606):
//! * `+1`: Return up to one target creature card from your graveyard to
//!   your hand. — GAP'd: graveyard-targeting needs a concrete
//!   `Zone::Graveyard(player)` target; no any-graveyard sentinel is in the
//!   demonstrated TargetRequirement surface.
//! * `−3`: Destroy target creature. Its controller loses 2 life. — destroy
//!   EXPRESSED on the target creature; the "its controller loses 2 life"
//!   rider needs the destroyed object's controller, which the demonstrated
//!   surface doesn't expose, so it is omitted.
//! * `−7`: Target opponent loses 2 life for each creature card in their
//!   graveyard. — GAP'd: a dynamic life-loss amount derived from a
//!   graveyard count is not in the demonstrated surface.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liliana, Death Mage");
    let liliana = reg.interner_mut().intern("Liliana");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(liliana);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Return up to one target creature card from your \
                       graveyard to your hand.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Destroy target creature. Its controller loses 2 \
                       life.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_destroy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Target opponent loses 2 life for each creature card \
                       in their graveyard.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_opponent()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven,
            }),
    )
}

/// `+1`
fn plus_one(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: graveyard-targeting return; no any-graveyard target sentinel in
    // the demonstrated surface.
    Vec::new()
}

/// `−3: Destroy target creature.` (the "controller loses 2 life" rider is
/// omitted — no demonstrated way to read the destroyed object's controller).
fn minus_three_destroy(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: *id }]
}

/// `−7`
fn minus_seven(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: dynamic life loss = 2 × (creature cards in target opponent's
    // graveyard); dynamic-amount LoseLife from a graveyard count is not in
    // the demonstrated surface.
    Vec::new()
}
