//! Gideon, Champion of Justice — `{2}{W}{W}` Legendary Planeswalker — Gideon, starting loyalty 1.
//!
//! +1: Put a loyalty counter on Gideon for each creature target opponent
//!   controls. IMPLEMENTED via AddCounters(Loyalty) on the source with a
//!   resolution-time count of the targeted opponent's creatures. (This is
//!   IN ADDITION to the +1 from the loyalty cost itself.)
//! 0: Until end of turn, Gideon becomes a Human Soldier creature with
//!   power and toughness each equal to the number of loyalty counters on
//!   him and gains indestructible. He's still a planeswalker. Prevent all
//!   damage that would be dealt to him this turn. IMPLEMENTED via PW
//!   animation — AddType(CREATURE) + SetBasePT(loyalty/loyalty) +
//!   GrantKeyword(Indestructible) + PreventDamage(self). P/T is snapshot
//!   at resolution (the demonstrated SetBasePT takes fixed values, not a
//!   live characteristic-defining value). Human/Soldier subtypes — GAP.
//! −15: Exile all other permanents. IMPLEMENTED via a ForEach exiling
//!   every battlefield permanent except Gideon himself.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
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
                text: "+1: Put a loyalty counter on Gideon, Champion of Justice \
                       for each creature target opponent controls.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_counters,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Until end of turn, Gideon, Champion of Justice becomes a \
                       Human Soldier creature with power and toughness each equal \
                       to the number of loyalty counters on him and gains \
                       indestructible. He's still a planeswalker. Prevent all \
                       damage that would be dealt to him this turn.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_animate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−15: Exile all other permanents.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 15)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_fifteen_exile,
            }),
    )
}

/// `+1` — put a loyalty counter on Gideon per creature target opponent controls.
fn plus_one_counters(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(opp)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // Creatures the targeted opponent controls (count_matching's `you`
    // parameter is the reference for ControllerConstraint::You).
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &filter, *opp);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Loyalty,
        count: n,
    }]
}

/// `0` — becomes a Human Soldier with P/T = loyalty counters, indestructible,
/// still a planeswalker, and prevent all damage to him this turn.
fn zero_animate(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Snapshot loyalty at resolution for the set P/T (SetBasePT takes a
    // fixed value, not a live characteristic-defining amount).
    let loyalty = state
        .objects
        .get(ctx.source)
        .map(|o| o.count_counters(CounterKind::Loyalty))
        .unwrap_or(0) as i32;
    // GAP: Human/Soldier subtype grants have no demonstrated Effect.
    vec![
        Effect::AddType {
            target: ctx.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: loyalty,
            toughness: loyalty,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        },
        Effect::PreventDamage {
            target: DamageTarget::Object(ctx.source),
            amount: None,
            duration: ReplacementDuration::EndOfTurn,
        },
    ]
}

/// `−15` — exile all other permanents.
fn minus_fifteen_exile(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    script::ids_matching(state, &ObjectFilter::permanent(), ctx.controller)
        .into_iter()
        .filter(|id| *id != ctx.source)
        .map(|id| Effect::ExilePermanent { target: id })
        .collect()
}
