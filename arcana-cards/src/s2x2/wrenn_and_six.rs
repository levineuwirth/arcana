//! Wrenn and Six — `{R}{G}` Legendary Planeswalker — Wrenn, starting loyalty 3.
//! Colors R/G.
//!
//! +1: Return up to one target land card from your graveyard to your hand
//!   (`Effect::ReturnFromGraveyardToHand`; target = land card in your graveyard).
//! −1: Wrenn and Six deals 1 damage to any target (`Effect::DealDamage`).
//! −7: emblem ("Instant and sorcery cards in your graveyard have retrace."). GAP:
//!   "retrace" is a rule-altering static permission (cast-from-graveyard cost
//!   rider) with no anthem/keyword builder. Emblem shell created with no grant.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wrenn and Six");
    let wrenn = reg.interner_mut().intern("Wrenn");
    let _emblem = reg.interner_mut().intern("Wrenn and Six emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wrenn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Return up to one target land card from your \
                       graveyard to your hand.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
                    },
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_return,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Wrenn and Six deals 1 damage to any target.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_damage,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: You get an emblem with \"Instant and sorcery cards \
                       in your graveyard have retrace.\"".into(),
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

fn plus_one_return(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}

fn minus_one_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let dt = match ctx.targets.targets.first() {
        Some(TargetChoice::Object(id)) => DamageTarget::Object(*id),
        Some(TargetChoice::Player(p)) => DamageTarget::Player(*p),
        Some(TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id))) => DamageTarget::Object(*id),
        Some(TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p))) => DamageTarget::Player(*p),
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage { source: ctx.source, target: dt, amount: 1 }]
}

fn minus_seven_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Wrenn and Six emblem").expect("emblem interned");
    // GAP: "instant and sorcery cards in your graveyard have retrace" — a
    //      rule-altering cast-from-graveyard permission, not an anthem/keyword.
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: Vec::new(),
        },
    }]
}
