//! Kaya, Geist Hunter — `{1}{W}{B}` Legendary Planeswalker — Kaya.
//! Starting loyalty inferred 5.
//! +1: Creatures you control gain deathtouch until end of turn. Put a +1/+1
//!   counter on up to one target creature token you control.
//! −2: Until end of turn, if one or more tokens would be created under your
//!   control, twice that many of those tokens are created instead.
//! −6: Exile all cards from all graveyards, then create a 1/1 white Spirit
//!   creature token with flying for each card exiled this way.
//!
//! GAP: −2 token-doubling replacement effect — no "double the tokens created"
//!   replacement in the demonstrated surface. Declared with correct −2 cost,
//!   effect GAP'd.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaya, Geist Hunter");
    let kaya = reg.interner_mut().intern("Kaya");
    let _spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kaya);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Creatures you control gain deathtouch until end of turn. Put \
                       a +1/+1 counter on up to one target creature token you control.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .tokens_only()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Until end of turn, if one or more tokens would be created \
                       under your control, twice that many of those tokens are created \
                       instead.".into(),
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
                effect: minus_two_double,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: Exile all cards from all graveyards, then create a 1/1 white \
                       Spirit creature token with flying for each card exiled this way.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six,
            }),
    )
}

fn plus_one(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let your_creatures = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &your_creatures, ctx.controller);
    let mut effects: Vec<Effect> = ids
        .into_iter()
        .map(|id| Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Deathtouch,
            duration: Duration::EndOfTurn,
        })
        .collect();
    if let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() {
        effects.push(Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
    }
    effects
}

fn minus_two_double(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: token-creation doubling replacement effect not expressible.
    Vec::new()
}

fn minus_six(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spirit = reg.interner().lookup("Spirit").expect("Spirit interned");
    let mut effects = Vec::new();
    let mut exiled = 0u32;
    for p in 0..state.num_players() {
        let ids: Vec<_> = state
            .objects
            .objects_in_zone(Zone::Graveyard(p))
            .map(|o| o.id)
            .collect();
        for id in ids {
            effects.push(Effect::ExileFromGraveyard { target: id });
            exiled += 1;
        }
    }
    for _ in 0..exiled {
        let mut token_subtypes = SubtypeSet::default();
        token_subtypes.0.insert(spirit);
        let token = TokenDefinition {
            name: spirit,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        };
        effects.push(Effect::CreateToken { controller: ctx.controller, token });
    }
    effects
}
