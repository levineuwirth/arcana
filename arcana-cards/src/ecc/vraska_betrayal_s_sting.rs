//! Vraska, Betrayal's Sting — `{4}{B}{B/P}` legendary planeswalker,
//! starting loyalty 7. Subtype Vraska; mono-black. (Compleated — the
//! "pay 2 life, enter with two fewer loyalty" rider isn't expressible by
//! the fixed printed-loyalty field; modeled as full printed 7.)
//!
//! Loyalty abilities:
//! * `0`: You draw a card and lose 1 life. Proliferate. (Functional —
//!   `DrawCards` + `LoseLife` + `Proliferate`.)
//! * `−2`: Target creature becomes a Treasure artifact with the
//!   sacrifice-for-mana ability and loses all other types and abilities.
//!   GAP — "becomes a Treasure with a granted activated ability" is bespoke.
//! * `−9`: Target player gets poison counters up to nine. GAP — the
//!   difference (data-dependent count) isn't expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vraska, Betrayal's Sting");
    let vraska = reg.interner_mut().intern("Vraska");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vraska);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B/P}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(7),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: You draw a card and lose 1 life. Proliferate.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_draw_proliferate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Target creature becomes a Treasure artifact with \
                       \"{T}, Sacrifice this artifact: Add one mana of any \
                       color\" and loses all other card types and abilities."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−9: If target player has fewer than nine poison \
                       counters, they get a number of poison counters equal \
                       to the difference."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 9)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_nine_gap,
            }),
    )
}

fn zero_draw_proliferate(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::LoseLife { player: ctx.controller, amount: 1 },
        Effect::Proliferate,
    ]
}

fn minus_two_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "becomes a Treasure with a granted sac-for-mana activated
    // ability" is bespoke (no type-set + granted-activation primitive).
    Vec::new()
}

fn minus_nine_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: poison counters equal to (9 − current) is a data-dependent
    // amount, not expressible.
    Vec::new()
}
