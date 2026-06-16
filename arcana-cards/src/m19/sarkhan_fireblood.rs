//! Sarkhan, Fireblood — `{1}{R}{R}` Legendary Planeswalker — Sarkhan.
//! Starting loyalty inferred 4.
//! +1: You may discard a card. If you do, draw a card.
//! +1: Add two mana in any combination of colors. Spend this mana only to
//!   cast Dragon spells.
//! −7: Create four 5/5 red Dragon creature tokens with flying.
//!
//! GAP: first +1 "you may discard a card; if you do, draw a card" — the
//!   discard is an optional cost gating the draw; OptionalPayment only models
//!   Mana/Life costs, so a discard-gated draw is not expressible. Declared
//!   with the correct +1 cost, effect GAP'd.
//! GAP: second +1 "add two mana in any combination of colors, spend only to
//!   cast Dragon spells" — both the free color choice and the Dragon-SUBTYPE
//!   spend restriction (SpendRestriction has no subtype variant) are not
//!   expressible. Declared with the correct +1 cost, effect GAP'd.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sarkhan, Fireblood");
    let sarkhan = reg.interner_mut().intern("Sarkhan");
    let _dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sarkhan);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
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
                text: "+1: You may discard a card. If you do, draw a card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_loot,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Add two mana in any combination of colors. Spend this mana \
                       only to cast Dragon spells.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: Create four 5/5 red Dragon creature tokens with flying.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_dragons,
            }),
    )
}

fn plus_one_loot(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: optional discard gating a draw not expressible (OptionalPayment is
    // Mana/Life only).
    Vec::new()
}

fn plus_one_mana(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: free color choice + Dragon-subtype spend restriction not expressible.
    Vec::new()
}

fn minus_seven_dragons(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dragon = reg.interner().lookup("Dragon").expect("Dragon interned");
    let make = || {
        let mut token_subtypes = SubtypeSet::default();
        token_subtypes.0.insert(dragon);
        let token = TokenDefinition {
            name: dragon,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        };
        Effect::CreateToken { controller: ctx.controller, token }
    };
    vec![make(), make(), make(), make()]
}
