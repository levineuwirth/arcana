//! Ajani Unyielding — `{4}{G}{W}` Legendary Planeswalker — Ajani, starting
//! loyalty 4. Colors G, W.
//!
//! +2: Reveal the top three cards of your library. Put all nonland
//!     permanent cards revealed this way into your hand and the rest on the
//!     bottom of your library in any order. (Partial — see resolver: only a
//!     single nonland permanent card is taken; "put ALL" isn't expressible.)
//! −2: Exile target creature. Its controller gains life equal to its power.
//! −9: Put five +1/+1 counters on each creature you control and five
//!     loyalty counters on each other planeswalker you control.

use arcana_core::effects::{DigRest, Effect};
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
    let name = reg.interner_mut().intern("Ajani Unyielding");
    let ajani = reg.interner_mut().intern("Ajani");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ajani);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Reveal the top three cards of your library. Put all nonland permanent cards revealed this way into your hand and the rest on the bottom of your library in any order.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Exile target creature. Its controller gains life equal to its power.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_exile_lifegain,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-9: Put five +1/+1 counters on each creature you control and five loyalty counters on each other planeswalker you control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 9)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_nine_counters,
            }),
    )
}

fn plus_two_dig(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // PARTIAL: oracle puts ALL revealed nonland permanent cards into hand;
    //          DigTopN takes a SINGLE matching card. The "put all" multi-take
    //          isn't expressible from the demonstrated surface.
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 3,
        filter: Some(ObjectFilter {
            types_any: Some(TypeLine(
                TypeLine::ARTIFACT
                    | TypeLine::CREATURE
                    | TypeLine::ENCHANTMENT
                    | TypeLine::PLANESWALKER,
            )),
            not_types: Some(TypeLine::LAND.into()),
            ..Default::default()
        }),
        rest: DigRest::BottomRandom,
    }]
}

fn minus_two_exile_lifegain(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let id = *id;
    // Read the creature's controller + power BEFORE it leaves the battlefield.
    let Some(obj) = state.objects.get(id) else { return Vec::new(); };
    let controller = obj.controller;
    let power = state.computed_power(id).unwrap_or(0).max(0) as u32;
    vec![
        Effect::ExilePermanent { target: id },
        Effect::GainLife { player: controller, amount: power },
    ]
}

fn minus_nine_counters(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut out = Vec::new();

    let creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    for id in creatures {
        out.push(Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 5,
        });
    }

    let planeswalkers = script::ids_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::PLANESWALKER.into())
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    for id in planeswalkers {
        if id == ctx.source {
            continue; // "each OTHER planeswalker you control"
        }
        out.push(Effect::AddCounters {
            target: id,
            kind: CounterKind::Loyalty,
            count: 5,
        });
    }

    out
}
