//! Chandra, Flamecaller — `{4}{R}{R}` Legendary Planeswalker — Chandra, starting loyalty 4.
//!
//! +1: Create two 3/1 red Elemental creature tokens with haste. Exile
//!   them at the beginning of the next end step. IMPLEMENTED via two
//!   CreateTokenSacEot (the engine schedules a next-end-step removal; it
//!   routes through sacrifice rather than exile — outcome-faithful, the
//!   tokens cease to exist). Elemental subtype IS expressible on tokens.
//! 0: Discard all the cards in your hand, then draw that many cards plus
//!   one. IMPLEMENTED via a resolution-time Discard of the whole hand
//!   then DrawCards of (hand size + 1).
//! −X: Chandra deals X damage to each creature. IMPLEMENTED via dynamic-X
//!   loyalty (remove_loyalty_x) dealing X to every battlefield creature.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Flamecaller");
    let chandra = reg.interner_mut().intern("Chandra");
    let elemental = reg.interner_mut().intern("Elemental");
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

    let _ = elemental;

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create two 3/1 red Elemental creature tokens with \
                       haste. Exile them at the beginning of the next end step."
                    .into(),
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
                effect: plus_one_tokens,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Discard all the cards in your hand, then draw that many \
                       cards plus one.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_wheel,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−X: Chandra deals X damage to each creature.".into(),
                cost: ActivationCost {
                    remove_loyalty_x: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_x_damage_each,
            }),
    )
}

/// `+1` — create two 3/1 red Elemental tokens with haste, exiled next end step.
fn plus_one_tokens(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elemental = reg.interner().lookup("Elemental").expect("Elemental interned");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(elemental);
    let token = TokenDefinition {
        name: elemental,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        abilities: vec![],
    };
    vec![
        Effect::CreateTokenSacEot { controller: ctx.controller, token: token.clone() },
        Effect::CreateTokenSacEot { controller: ctx.controller, token },
    ]
}

/// `0` — discard your hand, then draw that many cards plus one.
fn zero_wheel(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::hand_size(state, ctx.controller);
    vec![
        Effect::Discard {
            player: ctx.controller,
            count: n,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards { player: ctx.controller, count: n + 1 },
    ]
}

/// `−X` — deal X damage to each creature.
fn minus_x_damage_each(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = ctx.x_value.unwrap_or(0);
    let filter = ObjectFilter::creature();
    script::ids_matching(state, &filter, ctx.controller)
        .into_iter()
        .map(|id| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(id),
            amount: x,
        })
        .collect()
}
