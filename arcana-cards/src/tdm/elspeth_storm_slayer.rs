//! Elspeth, Storm Slayer — `{3}{W}{W}` legendary planeswalker, starting loyalty 5.
//!
//! Static: "If one or more tokens would be created under your control,
//! twice that many of those tokens are created instead." — a continuous
//! token-doubling replacement effect, NOT a loyalty ability; not
//! expressible from the demonstrated loyalty-ability surface, so omitted.
//!
//! Loyalty abilities (CR 606):
//! * `+1`: Create a 1/1 white Soldier creature token. — EXPRESSED.
//! * `0`: Put a +1/+1 counter on each creature you control. Those
//!   creatures gain flying until your next turn. — GAP'd (board-wide
//!   "each creature you control" sweep + duration keyword grant are not
//!   in the demonstrated single-target Effect surface).
//! * `−3`: Destroy target creature an opponent controls with mana value 3
//!   or greater. — destroy EXPRESSED on a target creature; the
//!   "an opponent controls / mana value ≥ 3" target restriction is not in
//!   the demonstrated TargetRequirement surface (approximated as plain
//!   target creature).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::effects::TokenDefinition;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elspeth, Storm Slayer");
    let elspeth = reg.interner_mut().intern("Elspeth");
    let _soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elspeth);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 1/1 white Soldier creature token.".into(),
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
                effect: make_soldier,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Put a +1/+1 counter on each creature you control. \
                       Those creatures gain flying until your next turn.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_anthem,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Destroy target creature an opponent controls with \
                       mana value 3 or greater.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_destroy,
            }),
    )
}

/// `+1: Create a 1/1 white Soldier creature token.`
fn make_soldier(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let soldier = reg.interner().lookup("Soldier").expect("interned Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(soldier);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: soldier,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

/// `0`
fn zero_anthem(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: board-wide "+1/+1 counter on EACH creature you control" sweep
    // plus a "gain flying until your next turn" duration keyword grant —
    // the demonstrated AddCounters / keyword-grant surface is single-target
    // only. Marker keyword import retained intentionally unused below.
    let _ = KeywordAbility::Flying;
    Vec::new()
}

/// `−3`
fn minus_three_destroy(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    // NOTE: oracle restricts to "creature an opponent controls with mana
    // value 3 or greater"; that target filter is not in the demonstrated
    // TargetRequirement surface, so the destroy is on a plain target
    // creature.
    vec![Effect::DestroyPermanent { target: *id }]
}
