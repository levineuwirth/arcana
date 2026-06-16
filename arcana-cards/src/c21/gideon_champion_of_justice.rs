//! Gideon, Champion of Justice — `{2}{W}{W}` Legendary Planeswalker —
//! Gideon, starting loyalty 1.
//!
//! +1: Put a loyalty counter on Gideon for each creature target opponent
//!     controls.
//! 0: Until end of turn, Gideon becomes a Human Soldier creature with
//!    power and toughness each equal to the number of loyalty counters on
//!    him and gains indestructible. He's still a planeswalker. Prevent all
//!    damage that would be dealt to him this turn. (GAP — animation +
//!    dynamic P/T + self damage prevention bundle.)
//! −15: Exile all other permanents.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gideon, Champion of Justice");
    let gideon = reg.interner_mut().intern("Gideon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gideon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(1),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Put a loyalty counter on Gideon for each creature target opponent controls.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_counters,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Until end of turn, Gideon becomes a Human Soldier creature with power and toughness each equal to the number of loyalty counters on him and gains indestructible. He's still a planeswalker. Prevent all damage that would be dealt to him this turn.".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_animate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-15: Exile all other permanents.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 15)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_fifteen_exile_all,
            }),
    )
}

fn plus_one_counters(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // The +1 cost already added one loyalty counter. The ability text adds
    // ONE counter per creature the target opponent controls (in addition to
    // the cost). Count those creatures and add that many counters.
    let Some(TargetChoice::Player(opp)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let creature_filter =
        ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &creature_filter, *opp);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Loyalty,
        count: n,
    }]
}

fn zero_animate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes a Human Soldier creature with P/T equal to loyalty
    //      counters and gains indestructible, still a planeswalker; prevent
    //      all damage to him this turn." The planeswalker-animation +
    //      dynamic-loyalty-P/T + bundled self damage prevention is not
    //      expressible from the demonstrated effect surface.
    Vec::new()
}

fn minus_fifteen_exile_all(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Exile all OTHER permanents (everything except Gideon himself).
    let all = script::ids_matching(state, &ObjectFilter::permanent(), ctx.controller);
    all.into_iter()
        .filter(|&id| id != ctx.source)
        .map(|id| Effect::ExilePermanent { target: id })
        .collect()
}
