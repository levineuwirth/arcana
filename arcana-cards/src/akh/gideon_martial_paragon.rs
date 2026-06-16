//! Gideon, Martial Paragon — `{4}{W}` Legendary Planeswalker — Gideon,
//! starting loyalty 5. Mono-white.
//!
//! Oracle text:
//! * `+2`: Untap all creatures you control. Those creatures get +1/+1
//!   until end of turn.
//! * `0`: Until end of turn, Gideon becomes a 5/5 Human Soldier creature
//!   with indestructible that's still a planeswalker. Prevent all damage
//!   that would be dealt to him this turn.
//! * `−10`: Creatures you control get +2/+2 until end of turn. Tap all
//!   creatures your opponents control.
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities.
//! * CR 606.3 — controller-only, sorcery speed, stack empty, once per
//!   turn per planeswalker.
//! * CR 704.5i — 0-loyalty SBA sacrifice.
//!
//! # Scope
//!
//! * `+2` untap-all + pump-all and `−10` pump-all + tap-all both require
//!   a board-wide (no-target) sweep of "creatures you control"/"creatures
//!   opponents control"; the demonstrated surface offers single-target
//!   Pump/Tap/Untap via TargetRequirement, not a mass non-targeted sweep
//!   Effect — GAP'd as best-effort shells with the correct loyalty cost.
//! * `0` "becomes a creature" is a self-animation continuous effect with
//!   a damage-prevention rider; not in the demonstrated Effect surface —
//!   GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gideon, Martial Paragon");
    let gideon = reg.interner_mut().intern("Gideon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gideon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Untap all creatures you control. Those creatures \
                       get +1/+1 until end of turn.".into(),
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
                effect: plus_two_untap_pump,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Until end of turn, Gideon, Martial Paragon becomes \
                       a 5/5 Human Soldier creature with indestructible \
                       that's still a planeswalker. Prevent all damage that \
                       would be dealt to him this turn.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_become_creature,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-10: Creatures you control get +2/+2 until end of \
                       turn. Tap all creatures your opponents control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 10)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_ten_pump_tap,
            }),
    )
}

/// `+2: Untap all creatures you control. Those creatures get +1/+1 until
/// end of turn.`
fn plus_two_untap_pump(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: board-wide non-targeted untap + pump of all creatures you
    // control is not in the demonstrated Effect surface.
    Vec::new()
}

/// `0: Gideon becomes a 5/5 Human Soldier creature with indestructible.`
fn zero_become_creature(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: self-animation continuous effect + damage-prevention rider not
    // expressible with the demonstrated Effect surface.
    Vec::new()
}

/// `-10: Creatures you control get +2/+2 until end of turn. Tap all
/// creatures your opponents control.`
fn minus_ten_pump_tap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: board-wide non-targeted pump + mass tap not in the
    // demonstrated Effect surface.
    Vec::new()
}
