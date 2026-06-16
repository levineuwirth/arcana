//! Nahiri, the Lithomancer — `{3}{W}{W}` Legendary Planeswalker — Nahiri,
//! starting loyalty 3.
//!
//! Loyalty abilities:
//! * `+2`: Create a 1/1 white Kor Soldier creature token. You may attach an
//!   Equipment you control to it. PARTIAL — the token is wired; the "may attach
//!   an Equipment" rider (needs the new token's id + a chosen Equipment) is not
//!   expressible (documented).
//! * `−2`: You may put an Equipment card from your hand or graveyard onto the
//!   battlefield. GAP — a "from hand OR graveyard" put isn't expressible
//!   (`PutFromHandOntoBattlefield` is hand-only; no two-zone may-put surface).
//!   Ability shell declared.
//! * `−10`: Create a colorless Equipment artifact token named Stoneforged Blade
//!   with indestructible, a +5/+5 + double strike equip effect, and equip {0}.
//!   GAP — a bespoke Equipment token carrying an equip ability + static
//!   equipped-creature buff is not expressible. Ability shell declared.
//!
//! ("Nahiri, the Lithomancer can be your commander" is a deck-construction
//! rule, not a loyalty ability — omitted.)

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nahiri, the Lithomancer");
    let nahiri = reg.interner_mut().intern("Nahiri");
    let _kor = reg.interner_mut().intern("Kor");
    let _soldier = reg.interner_mut().intern("Soldier");
    let _kor_soldier = reg.interner_mut().intern("Kor Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nahiri);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Create a 1/1 white Kor Soldier creature token. You \
                       may attach an Equipment you control to it.".into(),
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
                effect: plus_two_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: You may put an Equipment card from your hand or \
                       graveyard onto the battlefield.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−10: Create a colorless Equipment artifact token named \
                       Stoneforged Blade. It has indestructible, \"Equipped \
                       creature gets +5/+5 and has double strike,\" and equip \
                       {0}.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 10)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ultimate,
            }),
    )
}

/// `+2: Create a 1/1 white Kor Soldier creature token.` (Attach rider GAP'd.)
fn plus_two_token(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    // PARTIAL: "you may attach an Equipment you control to it" — needs the new
    // token's id + a chosen Equipment; not expressible. Token wired.
    let kor = reg.interner().lookup("Kor").expect("Kor interned");
    let soldier = reg.interner().lookup("Soldier").expect("Soldier interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kor);
    subtypes.0.insert(soldier);
    let token = TokenDefinition {
        name: reg.interner().lookup("Kor Soldier").expect("Kor Soldier interned"),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

/// `−2`: may put an Equipment from hand or graveyard onto the battlefield.
fn minus_two(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "from your hand OR graveyard" two-zone may-put is not expressible.
    Vec::new()
}

/// `−10`: Stoneforged Blade Equipment token.
fn ultimate(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: bespoke Equipment token with an equip ability + a static
    // equipped-creature +5/+5 / double strike buff is not expressible.
    Vec::new()
}
