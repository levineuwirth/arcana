//! Nissa, Voice of Zendikar — `{1}{G}{G}` legendary planeswalker, starting loyalty 3.
//!
//! +1: Create a 0/1 green Plant creature token.
//! −2: Put a +1/+1 counter on each creature you control.
//! −7: You gain X life and draw X cards, where X is the number of lands
//!     you control.
//!
//! Scope: all three abilities are fully expressed — token creation, a
//! per-creature counter (ForEach over your battlefield creatures), and a
//! lands-scaled gain-life + draw.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nissa, Voice of Zendikar");
    let nissa = reg.interner_mut().intern("Nissa");
    let _plant = reg.interner_mut().intern("Plant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nissa);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
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
                text: "+1: Create a 0/1 green Plant creature token.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_plant,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Put a +1/+1 counter on each creature you control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_counters,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: You gain X life and draw X cards, where X is the \
                       number of lands you control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_landfall,
            }),
    )
}

/// `+1: Create a 0/1 green Plant token.`
fn plus_one_plant(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let plant = reg.interner().lookup("Plant").expect("interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: plant,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

/// `−2: Put a +1/+1 counter on each creature you control.`
fn minus_two_counters(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: arcana_core::objects::NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
    }]
}

/// `−7: gain X life and draw X, X = lands you control.`
fn minus_seven_landfall(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let land_filter = ObjectFilter::permanent()
        .with_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You);
    let x = script::count_matching(state, &land_filter, ctx.controller);
    vec![
        Effect::GainLife { player: ctx.controller, amount: x },
        Effect::DrawCards { player: ctx.controller, count: x },
    ]
}
