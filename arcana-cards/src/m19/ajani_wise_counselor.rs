//! Ajani, Wise Counselor — `{3}{W}{W}` Legendary Planeswalker — Ajani,
//! starting loyalty 6.
//!
//! Oracle text:
//! * `+2`: You gain 1 life for each creature you control.
//! * `−3`: Creatures you control get +2/+2 until end of turn.
//! * `−9`: Put X +1/+1 counters on target creature, where X is your
//!   life total.
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities (activated abilities whose cost is
//!   adding/removing loyalty counters).
//! * CR 704.5i — 0-loyalty state-based sacrifice.
//!
//! # Scope
//!
//! * `+2` is modeled: GainLife with the resolution-time amount equal to
//!   the number of creatures you control (`script::count_matching`).
//! * `−3` is GAP'd: a board-wide, untargeted pump of every creature you
//!   control is not expressible with the demonstrated single-target
//!   `Effect::Pump` surface.
//! * `−9` is GAP'd: the counter count is dynamic (your life total), and
//!   `Effect::AddCounters { count }` is a fixed amount — not expressible.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ajani, Wise Counselor");
    let ajani = reg.interner_mut().intern("Ajani");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ajani);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(6),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: You gain 1 life for each creature you control.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_gain_life,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Creatures you control get +2/+2 until end of turn.".into(),
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
                effect: minus_three_pump,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−9: Put X +1/+1 counters on target creature, where X is \
                       your life total.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 9)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_nine_counters,
            }),
    )
}

/// `+2: You gain 1 life for each creature you control.`
fn plus_two_gain_life(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let you = ctx.controller;
    let filter = ObjectFilter::creature();
    let n = script::count_matching(state, &filter, you);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::GainLife {
        player: you,
        amount: n,
    }]
}

/// `−3: Creatures you control get +2/+2 until end of turn.`
fn minus_three_pump(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: board-wide, untargeted pump of every creature you control is
    // not expressible with the demonstrated single-target Effect::Pump.
    Vec::new()
}

/// `−9: Put X +1/+1 counters on target creature, where X is your life total.`
fn minus_nine_counters(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: counter count is dynamic (your life total); AddCounters takes a
    // fixed count.
    Vec::new()
}
