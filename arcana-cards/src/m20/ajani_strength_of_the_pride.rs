//! Ajani, Strength of the Pride — `{2}{W}{W}` Legendary Planeswalker — Ajani,
//! starting loyalty 5 — colors W.
//!
//! Oracle text:
//! * `+1`: You gain life equal to the number of creatures and planeswalkers
//!   you control. — `GainLife` with a dynamic amount.
//! * `−2`: Create a 2/2 white Cat Soldier creature token named Ajani's
//!   Pridemate with "Whenever you gain life, put a +1/+1 counter on this
//!   creature." — `CreateToken` is faithful; the lifegain-counter triggered
//!   ability on the token is GAP'd (token still minted with no abilities).
//! * `0`: If you have at least 15 life more than your starting life total,
//!   exile Ajani and each artifact and creature your opponents control. —
//!   conditional mass exile referencing starting life total; GAP.
//!
//! # Rules references
//! * CR 606 — loyalty abilities.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ajani, Strength of the Pride");
    let ajani = reg.interner_mut().intern("Ajani");
    let _cat = reg.interner_mut().intern("Cat");
    let _soldier = reg.interner_mut().intern("Soldier");
    let _pridemate = reg.interner_mut().intern("Ajani's Pridemate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ajani);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: You gain life equal to the number of creatures and \
                       planeswalkers you control.".into(),
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
                effect: plus_one_gain,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Create a 2/2 white Cat Soldier creature token named \
                       Ajani's Pridemate with \"Whenever you gain life, put a \
                       +1/+1 counter on this creature.\"".into(),
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
                effect: minus_two_pridemate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: If you have at least 15 life more than your starting \
                       life total, exile Ajani and each artifact and creature \
                       your opponents control.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_exile,
            }),
    )
}

/// `+1`: gain life = creatures + planeswalkers you control.
fn plus_one_gain(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let creatures = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    let pws = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::PLANESWALKER.into())
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    let amount = creatures + pws;
    if amount == 0 {
        return Vec::new();
    }
    vec![Effect::GainLife { player: ctx.controller, amount }]
}

/// `−2`: create a 2/2 white Cat Soldier (lifegain-counter trigger GAP'd).
fn minus_two_pridemate(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the token's "Whenever you gain life, put a +1/+1 counter on this
    // creature" triggered ability is not inlined; the token is faithful.
    let token_name = match reg.interner().lookup("Ajani's Pridemate") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    if let Some(s) = reg.interner().lookup("Cat") {
        subtypes.0.insert(s);
    }
    if let Some(s) = reg.interner().lookup("Soldier") {
        subtypes.0.insert(s);
    }
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: token_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

/// `0`: conditional mass exile referencing starting life total.
fn zero_exile(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if you have at least 15 life more than your starting life total"
    // references the starting life total, and the conditional self+mass exile
    // is not expressible from the demonstrated Effect surface.
    Vec::new()
}
