//! Vraska, Relic Seeker — `{4}{B}{G}` Legendary Planeswalker — Vraska,
//! starting loyalty 6 — colors B, G.
//!
//! Oracle text:
//! * `+2`: Create a 2/2 black Pirate creature token with menace. —
//!   `CreateToken` with `KeywordAbility::Menace`.
//! * `−3`: Destroy target artifact, creature, or enchantment. If that
//!   permanent was a creature, create a Treasure token. — modeled as
//!   `DestroyPermanent` + `CreateCommodityToken` (Treasure) unconditionally;
//!   the "was a creature" gate on the Treasure is GAP'd.
//! * `−10`: Target player's life total becomes 1. — `SetLifeTotal` to 1.
//!
//! # Rules references
//! * CR 606 — loyalty abilities.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vraska, Relic Seeker");
    let vraska = reg.interner_mut().intern("Vraska");
    let _pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vraska);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(6),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Create a 2/2 black Pirate creature token with menace."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_pirate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Destroy target artifact, creature, or enchantment. \
                       If that permanent was a creature, create a Treasure \
                       token.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types_any(
                            (TypeLine::ARTIFACT
                                | TypeLine::CREATURE
                                | TypeLine::ENCHANTMENT)
                                .into(),
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
                effect: minus_three_destroy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−10: Target player's life total becomes 1.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 10)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_ten_set_life,
            }),
    )
}

/// `+2`: create a 2/2 black Pirate with menace.
fn plus_two_pirate(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let token_name = match reg.interner().lookup("Pirate") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(token_name);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: token_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Menace],
            abilities: vec![],
        },
    }]
}

/// `−3`: destroy target artifact/creature/enchantment, then make a Treasure.
fn minus_three_destroy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "if that permanent was a creature" gate on the Treasure is not
    // modeled — a Treasure is created unconditionally.
    let target = match ctx.targets.targets.first() {
        Some(TargetChoice::Object(id)) => *id,
        _ => return Vec::new(),
    };
    vec![Effect::Sequence(vec![
        Effect::DestroyPermanent { target },
        Effect::CreateCommodityToken {
            controller: ctx.controller,
            kind: CommodityToken::Treasure,
            count: 1,
        },
    ])]
}

/// `−10`: target player's life total becomes 1.
fn minus_ten_set_life(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let player = match ctx.targets.targets.first() {
        Some(TargetChoice::Player(p)) => *p,
        _ => return Vec::new(),
    };
    vec![Effect::SetLifeTotal { player, amount: 1 }]
}
