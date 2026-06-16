//! Elspeth, Sun's Champion — `{4}{W}{W}` Legendary Planeswalker —
//! Elspeth, printed starting loyalty 4 (W).
//!
//! Oracle text:
//!   +1: Create three 1/1 white Soldier creature tokens.
//!   −3: Destroy all creatures with power 4 or greater.
//!   −7: You get an emblem with "Creatures you control get +2/+2 and
//!       have flying."
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities (the engine enforces timing / once per
//!   turn / 0-loyalty SBA).
//! * CR 704.5i — 0-loyalty state-based sacrifice.
//!
//! # Scope
//!
//! * `+1` — three `Effect::CreateToken` (CreateToken has no count
//!   field; the value is repeated).
//! * `−3` — sweep every creature with power ≥ 4 and destroy each via
//!   `Effect::ForEach` over `Effect::DestroyPermanent`.
//! * `−7` — emblem creation ("Creatures you control get +2/+2 and have
//!   flying") is a continuous-static emblem, GAP per the system prompt;
//!   the ability shell keeps its `−7` cost.

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
    let name = reg.interner_mut().intern("Elspeth, Sun's Champion");
    let elspeth = reg.interner_mut().intern("Elspeth");
    // "Soldier" is the token's creature subtype.
    let _soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elspeth);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
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
                text: "+1: Create three 1/1 white Soldier creature \
                       tokens.".into(),
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
                effect: plus_one_soldiers,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Destroy all creatures with power 4 or \
                       greater.".into(),
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
                effect: minus_three_wrath,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: You get an emblem with \"Creatures you control \
                       get +2/+2 and have flying.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_emblem,
            }),
    )
}

/// `+1: Create three 1/1 white Soldier creature tokens.`
fn plus_one_soldiers(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let soldier = reg.interner().lookup("Soldier")
        .expect("Soldier interned during register()");
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

/// `−3: Destroy all creatures with power 4 or greater.`
fn minus_three_wrath(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature().with_min_power(4);
    let creatures = script::ids_matching(state, &filter, ctx.controller);
    if creatures.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: creatures.clone(),
        effect: Box::new(Effect::DestroyPermanent { target: creatures[0] }),
    }]
}

/// `−7: You get an emblem with "Creatures you control get +2/+2 and have
/// flying."`
fn minus_seven_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem carrying a continuous +2/+2-and-flying anthem static
    // isn't expressible from the demonstrated Effect surface (emblem
    // abilities are triggered-ability shaped, not continuous statics).
    Vec::new()
}
