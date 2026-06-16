//! Jared Carthalion — `{W}{U}{B}{R}{G}` legendary planeswalker, starting loyalty 5.
//! Subtype Jared; all five colors.
//!
//! Loyalty abilities:
//! * `+1`: Create a 3/3 Kavu creature token with trample that's all colors.
//!   (Functional via `Effect::CreateToken`.)
//! * `−3`: Choose up to two target creatures. For each, put +1/+1 counters
//!   equal to the number of colors it is. GAP — counter count derived per
//!   target from that target's color count is not expressible.
//! * `−6`: Return target multicolored card from your graveyard to your hand;
//!   extra if all colors. GAP — graveyard-targeting (no any-graveyard
//!   sentinel target) plus the conditional Treasure/draw rider.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jared Carthalion");
    let jared = reg.interner_mut().intern("Jared");
    let _kavu = reg.interner_mut().intern("Kavu");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jared);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 3/3 Kavu creature token with trample \
                       that's all colors."
                    .into(),
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
                effect: plus_one_kavu,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Choose up to two target creatures. For each of \
                       them, put a number of +1/+1 counters on it equal to \
                       the number of colors it is."
                    .into(),
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
                effect: minus_three_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: Return target multicolored card from your \
                       graveyard to your hand. If that card was all colors, \
                       draw a card and create two Treasure tokens."
                    .into(),
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
                effect: minus_six_gap,
            }),
    )
}

fn plus_one_kavu(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let kavu = reg.interner().lookup("Kavu").expect("Kavu interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kavu);
    let token = TokenDefinition {
        name: kavu,
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn minus_three_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: per-target counter count equal to that target's color count
    // is not expressible (AddCounters count is fixed).
    Vec::new()
}

fn minus_six_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: graveyard-targeting (no any-graveyard target sentinel) plus a
    // conditional all-colors draw + Treasure rider.
    Vec::new()
}
