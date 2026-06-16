//! Tibalt, Wicked Tormentor — `{3}{R}{R}` Legendary Planeswalker — Tibalt,
//! starting loyalty 5.
//!
//! +1: Add {R}{R}. Draft a card from Tibalt, Wicked Tormentor's spellbook,
//!   then exile it. Until end of turn, you may cast that card.
//! +1: Tibalt, Wicked Tormentor deals 4 damage to target creature or
//!   planeswalker unless its controller has Tibalt deal 4 damage to them. If
//!   they do, you may discard a card. If you do, draw a card.
//! −X: Create X 1/1 red Devil creature tokens with "When this creature dies,
//!   it deals 1 damage to any target."
//!
//! GAP: the first +1's spellbook draft + "you may cast that card" rider is not
//!   expressible; only the "Add {R}{R}" half is emitted.
//! GAP: the second +1's "unless its controller has Tibalt deal 4 damage to
//!   them" is a bespoke unless/discard/draw clause not in the demonstrated
//!   surface; declared with its +1 cost and target, effect empty.
//! GAP: the −X ultimate uses a dynamic chosen X as its loyalty cost, which
//!   `remove_self_counter` (a fixed u32) cannot express — the ability is
//!   OMITTED entirely.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tibalt, Wicked Tormentor");
    let tibalt = reg.interner_mut().intern("Tibalt");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tibalt);

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
                text: "+1: Add {R}{R}. Draft a card from Tibalt, Wicked \
                       Tormentor's spellbook, then exile it. Until end of turn, \
                       you may cast that card.".into(),
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
                effect: plus_one_mana_spellbook,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Tibalt, Wicked Tormentor deals 4 damage to target \
                       creature or planeswalker unless its controller has \
                       Tibalt deal 4 damage to them. If they do, you may \
                       discard a card. If you do, draw a card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(
                            TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER),
                        ),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_unless_damage,
            }),
        // GAP: dynamic-X loyalty cost — the "−X: Create X 1/1 red Devil
        // creature tokens …" ability cannot be costed with a fixed
        // remove_self_counter and is omitted.
    )
}

/// `+1: Add {R}{R}. …`
fn plus_one_mana_spellbook(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: spellbook draft + cast-this-turn rider is not expressible; only the
    // "Add {R}{R}" half is emitted.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Red, ctx.source),
            ManaUnit::plain(ManaColor::Red, ctx.source),
        ],
    }]
}

/// `+1: Tibalt … deals 4 damage to target … unless …`
fn plus_one_unless_damage(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "unless its controller has Tibalt deal 4 damage to them; if
    // they do, you may discard a card; if you do, draw a card" clause is a
    // bespoke conditional not in the demonstrated Effect surface.
    Vec::new()
}
