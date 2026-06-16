//! Ajani Steadfast — `{3}{W}` Legendary Planeswalker — Ajani, starting loyalty 4.
//!
//! +1: Until end of turn, up to one target creature gets +1/+1 and gains first
//!   strike, vigilance, and lifelink.
//! −2: Put a +1/+1 counter on each creature you control and a loyalty counter on
//!   each other planeswalker you control.
//! −7: You get an emblem with "If a source would deal damage to you or a
//!   planeswalker you control, prevent all but 1 of that damage." Modeled as a
//!   CreateEmblem shell whose grant is a rule-altering damage-prevention
//!   replacement effect not expressible from the demonstrated static builders —
//!   GAP'd (correct −7 cost retained).

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::layers::Duration;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::script;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ajani Steadfast");
    let ajani = reg.interner_mut().intern("Ajani");
    let _emblem = reg.interner_mut().intern("Ajani Steadfast emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ajani);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
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
                text: "+1: Until end of turn, up to one target creature gets \
                       +1/+1 and gains first strike, vigilance, and lifelink.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature(),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_pump,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Put a +1/+1 counter on each creature you control and a \
                       loyalty counter on each other planeswalker you control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_counters,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: You get an emblem with \"If a source would deal damage \
                       to you or a planeswalker you control, prevent all but 1 of \
                       that damage.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_emblem,
            }),
    )
}

fn plus_one_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![
        Effect::Pump {
            target: *id,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::FirstStrike,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Vigilance,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Lifelink,
            duration: Duration::EndOfTurn,
        },
    ]
}

fn minus_two_counters(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let creatures = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let pws = ObjectFilter::permanent()
        .with_types(TypeLine::PLANESWALKER.into())
        .controlled_by(ControllerConstraint::You);
    let mut effects = Vec::new();
    for id in script::ids_matching(state, &creatures, ctx.controller) {
        effects.push(Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
    }
    for id in script::ids_matching(state, &pws, ctx.controller) {
        if id == ctx.source {
            continue;
        }
        effects.push(Effect::AddCounters {
            target: id,
            kind: CounterKind::Loyalty,
            count: 1,
        });
    }
    effects
}

fn minus_seven_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Ajani Steadfast emblem").expect("name interned");
    // GAP: "prevent all but 1 of that damage" is a rule-altering damage
    //      prevention/replacement effect not expressible from the demonstrated
    //      anthem/keyword/filtered static builders. Emblem shell retained.
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: Vec::new(),
        },
    }]
}
