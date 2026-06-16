//! Chandra, Flamecaller — `{4}{R}{R}` Legendary Planeswalker — Chandra,
//! starting loyalty 4.
//!
//! +1: Create two 3/1 red Elemental creature tokens with haste. Exile them at
//!     the beginning of the next end step.
//! 0: Discard all the cards in your hand, then draw that many cards plus one.
//! −X: Chandra deals X damage to each creature.
//!
//! GAP: −X is a dynamic-X loyalty cost ("−X") — `remove_self_counter` is a
//!   fixed u32 and cannot express the player-chosen X, so the ability is
//!   omitted entirely (per the dynamic-X rule).
//! Note: the +1 rider says "exile them at the next end step"; the engine's
//!   CreateTokenSacEot destroys (sacrifices) the tokens at the next end step
//!   instead — token-faithful (the tokens cease to exist either way) and the
//!   closest demonstrated primitive.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::effects::DiscardChoice;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Flamecaller");
    let chandra = reg.interner_mut().intern("Chandra");
    let _elemental = reg.interner_mut().intern("Elemental");
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
                text: "0: Discard all the cards in your hand, then draw that \
                       many cards plus one."
                    .into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_wheel,
            }),
    )
    // GAP: -X "Chandra deals X damage to each creature" omitted — dynamic-X
    // loyalty cost is not expressible.
}

fn elemental_token(reg: &CardRegistry) -> TokenDefinition {
    let elemental = reg.interner().lookup("Elemental").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    TokenDefinition {
        name: elemental,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        abilities: vec![],
    }
}

fn plus_one_tokens(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::CreateTokenSacEot {
            controller: ctx.controller,
            token: elemental_token(reg),
        },
        Effect::CreateTokenSacEot {
            controller: ctx.controller,
            token: elemental_token(reg),
        },
    ]
}

fn zero_wheel(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::hand_size(state, ctx.controller);
    let mut effects = Vec::new();
    if n > 0 {
        effects.push(Effect::Discard {
            player: ctx.controller,
            count: n,
            choice: DiscardChoice::ControllerChooses,
        });
    }
    effects.push(Effect::DrawCards {
        player: ctx.controller,
        count: n + 1,
    });
    effects
}
