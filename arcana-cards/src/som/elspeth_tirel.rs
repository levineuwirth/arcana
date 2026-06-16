//! Elspeth Tirel — `{3}{W}{W}` Legendary Planeswalker — Elspeth,
//! starting loyalty 4. White.
//!
//! Oracle text:
//! * `+2`: You gain 1 life for each creature you control.
//! * `−2`: Create three 1/1 white Soldier creature tokens.
//! * `−5`: Destroy all other permanents except for lands and tokens.
//!
//! # Scope
//!
//! * `+2` computes the gained life at resolution from the count of
//!   creatures you control (`script::count_matching`).
//! * `−2` mints three 1/1 white Soldier tokens.
//! * `−5` is a board-wide conditional destruction sweep (everything
//!   except lands, tokens, and Elspeth herself) — not expressible as a
//!   targeted effect from the surface; GAP'd, cost shell declared.

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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elspeth Tirel");
    let elspeth = reg.interner_mut().intern("Elspeth");
    let _soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elspeth);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: You gain 1 life for each creature you \
                       control.".into(),
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
                effect: plus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Create three 1/1 white Soldier creature \
                       tokens.".into(),
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
                text: "−5: Destroy all other permanents except for lands \
                       and tokens.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 5)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_five,
            }),
    )
}

/// `+2`: gain 1 life per creature you control.
fn plus_two(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::CREATURE.into())
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![Effect::GainLife { player: ctx.controller, amount: n }]
}

/// `−2`: create three 1/1 white Soldier tokens.
fn minus_two(_s: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let soldier = reg.interner().lookup("Soldier").expect("Soldier interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(soldier);
    let token = TokenDefinition {
        name: soldier,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: ctx.controller, token: token.clone() },
        Effect::CreateToken { controller: ctx.controller, token: token.clone() },
        Effect::CreateToken { controller: ctx.controller, token },
    ]
}

/// `−5`: destroy all other permanents except lands and tokens.
fn minus_five(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: conditional board-wide destruction sweep (all permanents
    // except lands, tokens, and the source) is not expressible as a
    // targeted destroy from the demonstrated surface.
    Vec::new()
}
