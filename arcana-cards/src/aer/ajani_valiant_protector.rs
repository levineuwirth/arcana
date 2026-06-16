//! Ajani, Valiant Protector — `{4}{G}{W}` Legendary Planeswalker —
//! Ajani, starting loyalty 4. G/W.
//!
//! Oracle text:
//! * `+2`: Put two +1/+1 counters on up to one target creature.
//! * `+1`: Reveal cards from the top of your library until you reveal a
//!   creature card. Put that card into your hand and the rest on the
//!   bottom of your library in a random order.
//! * `−11`: Put X +1/+1 counters on target creature, where X is your
//!   life total. That creature gains trample until end of turn.
//!
//! # Scope
//!
//! * `+2` puts two +1/+1 counters on up to one target creature.
//! * `+1` is `Effect::RevealUntil` (creature filter → hand, rest to
//!   bottom in random order).
//! * `−11` has a dynamic counter count (X = your life total) — not
//!   expressible (`AddCounters.count` is a fixed `u32`); GAP'd, shell
//!   declared with the correct `−11` cost.

use arcana_core::effects::{DigRest, Effect, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ajani, Valiant Protector");
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
                text: "+2: Put two +1/+1 counters on up to one target \
                       creature.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Reveal cards from the top of your library until \
                       you reveal a creature card. Put that card into your \
                       hand and the rest on the bottom of your library in a \
                       random order.".into(),
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
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−11: Put X +1/+1 counters on target creature, where \
                       X is your life total. That creature gains trample \
                       until end of turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 11)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eleven,
            }),
    )
}

/// `+2`: two +1/+1 counters on up to one target creature.
fn plus_two(_s: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![Effect::AddCounters { target: *id, kind: CounterKind::PlusOnePlusOne, count: 2 }]
}

/// `+1`: reveal until a creature → hand, rest to bottom (random).
fn plus_one(_s: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::RevealUntil {
        player: ctx.controller,
        filter: ObjectFilter::new().with_types(TypeLine::CREATURE.into()),
        found_dest: RevealDest::Hand,
        rest: DigRest::BottomRandom,
        max_reveal: None,
    }]
}

/// `−11`: X +1/+1 counters (X = your life total) + trample.
fn minus_eleven(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: dynamic counter count — X equals the controller's life total —
    // is not expressible (AddCounters.count is a fixed u32).
    Vec::new()
}
