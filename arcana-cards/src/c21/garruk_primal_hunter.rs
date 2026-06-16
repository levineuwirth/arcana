//! Garruk, Primal Hunter — `{2}{G}{G}{G}` Legendary Planeswalker — Garruk,
//! starting loyalty 3.
//!
//! Oracle text:
//! * `+1`: Create a 3/3 green Beast creature token.
//! * `−3`: Draw cards equal to the greatest power among creatures you
//!   control.
//! * `−6`: Create a 6/6 green Wurm creature token for each land you
//!   control.
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities (cost is adding/removing loyalty
//!   counters); CR 606.3 — sorcery speed, stack empty, controller-only,
//!   once per turn per planeswalker.
//! * CR 704.5i — a planeswalker with 0 loyalty is sacrificed as an SBA.
//!
//! # Scope
//!
//! All three loyalty abilities are expressed. The `−3` draw computes the
//! greatest power among the controller's creatures at resolution and the
//! `−6` creates one Wurm per land the controller controls (a runtime
//! count of `Effect::CreateToken` values), both via the resolution-time
//! `&GameState`.

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
    let name = reg.interner_mut().intern("Garruk, Primal Hunter");
    let garruk = reg.interner_mut().intern("Garruk");
    // Token creature subtypes, interned up front for the resolvers.
    let _beast = reg.interner_mut().intern("Beast");
    let _wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(garruk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 3/3 green Beast creature token.".into(),
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
                effect: plus_one_beast,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Draw cards equal to the greatest power among \
                       creatures you control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: Create a 6/6 green Wurm creature token for each \
                       land you control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_wurms,
            }),
    )
}

/// `+1: Create a 3/3 green Beast creature token.`
fn plus_one_beast(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let beast = reg.interner().lookup("Beast")
        .expect("Beast interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let token = TokenDefinition {
        name: beast,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

/// `−3: Draw cards equal to the greatest power among creatures you control.`
fn minus_three_draw(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You);
    let greatest = script::ids_matching(state, &filter, ctx.controller)
        .into_iter()
        .map(|id| script::power_of(state, id))
        .max()
        .unwrap_or(0);
    let count = greatest.max(0) as u32;
    if count == 0 {
        return Vec::new();
    }
    vec![Effect::DrawCards { player: ctx.controller, count }]
}

/// `−6: Create a 6/6 green Wurm creature token for each land you control.`
fn minus_six_wurms(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wurm = reg.interner().lookup("Wurm")
        .expect("Wurm interned during register()");
    let land_filter = ObjectFilter::permanent()
        .controlled_by(ControllerConstraint::You);
    let land_filter = ObjectFilter { types: Some(TypeLine::LAND.into()), ..land_filter };
    let lands = script::count_matching(state, &land_filter, ctx.controller);
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);
    let token = TokenDefinition {
        name: wurm,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..lands)
        .map(|_| Effect::CreateToken { controller: ctx.controller, token: token.clone() })
        .collect()
}
