//! Nahiri, Heir of the Ancients — `{2}{R}{W}` Legendary Planeswalker — Nahiri,
//! starting loyalty 4.
//!
//! Loyalty abilities:
//! * `+1`: Create a 1/1 white Kor Warrior creature token. You may attach an
//!   Equipment you control to it. PARTIAL — token wired; attach rider not
//!   expressible (documented).
//! * `−2`: Look at the top six cards of your library. You may reveal a Warrior
//!   or Equipment card from among them and put it into your hand. Put the rest
//!   on the bottom in a random order. (`DigTopN` count 6, filter = Warrior OR
//!   Equipment subtype, rest = bottom random.)
//! * `−3`: Nahiri deals damage to target creature or planeswalker equal to
//!   twice the number of Equipment you control. (resolution-time amount via
//!   `script::count_matching`.)

use arcana_core::effects::{DigRest, Effect, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nahiri, Heir of the Ancients");
    let nahiri = reg.interner_mut().intern("Nahiri");
    let _kor = reg.interner_mut().intern("Kor");
    let _warrior = reg.interner_mut().intern("Warrior");
    let _equipment = reg.interner_mut().intern("Equipment");
    let _kor_warrior = reg.interner_mut().intern("Kor Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nahiri);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 1/1 white Kor Warrior creature token. You \
                       may attach an Equipment you control to it.".into(),
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
                effect: plus_one_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Look at the top six cards of your library. You may \
                       reveal a Warrior or Equipment card from among them and \
                       put it into your hand. Put the rest on the bottom of \
                       your library in a random order.".into(),
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
                effect: minus_two_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Nahiri deals damage to target creature or \
                       planeswalker equal to twice the number of Equipment you \
                       control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_burn,
            }),
    )
}

/// `+1: Create a 1/1 white Kor Warrior creature token.` (Attach rider GAP'd.)
fn plus_one_token(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let kor = reg.interner().lookup("Kor").expect("Kor interned");
    let warrior = reg.interner().lookup("Warrior").expect("Warrior interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kor);
    subtypes.0.insert(warrior);
    let token = TokenDefinition {
        name: reg.interner().lookup("Kor Warrior").expect("Kor Warrior interned"),
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

/// `−2`: dig top six for a Warrior or Equipment card.
fn minus_two_dig(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let warrior = reg.interner().lookup("Warrior").expect("Warrior interned");
    let equipment = reg.interner().lookup("Equipment").expect("Equipment interned");
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 6,
        filter: Some(ObjectFilter::new().with_subtypes_any(vec![warrior, equipment])),
        rest: DigRest::BottomRandom,
    }]
}

/// `−3`: deal damage equal to twice the Equipment you control.
fn minus_three_burn(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => DamageTarget::Object(*id),
        _ => return Vec::new(),
    };
    let equipment = match reg.interner().lookup("Equipment") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let count = script::count_matching(
        state,
        &ObjectFilter::permanent().with_subtype_sym(equipment),
        ctx.controller,
    );
    vec![Effect::DealDamage { source: ctx.source, target: dt, amount: count * 2 }]
}
