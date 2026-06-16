//! Zariel, Archduke of Avernus — `{2}{R}{R}` Legendary Planeswalker —
//! Zariel, starting loyalty 4. Red.
//!
//! Oracle text:
//! * `+1`: Creatures you control get +1/+0 and gain haste until end of
//!   turn.
//! * `0`: Create a 1/1 red Devil creature token with "When this token
//!   dies, it deals 1 damage to any target."
//! * `−6`: You get an emblem with "At the end of the first combat phase
//!   on your turn, untap target creature you control. After this phase,
//!   there is an additional combat phase."
//!
//! # Scope
//!
//! * `+1` is expressed as a `+1/+0` anthem until end of turn; the "and
//!   gain haste" rider to all your creatures isn't part of the
//!   `Effect::Anthem` surface (P/T only) — noted partial.
//! * `0` mints the 1/1 red Devil token; its "when this dies, deal 1 to
//!   any target" triggered ability (a targeted death trigger on a token)
//!   isn't built here — the token body is faithful, the rider noted.
//! * `−6` grants a bespoke combat-phase emblem — GAP'd.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zariel, Archduke of Avernus");
    let zariel = reg.interner_mut().intern("Zariel");
    let _devil = reg.interner_mut().intern("Devil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zariel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Creatures you control get +1/+0 and gain haste \
                       until end of turn.".into(),
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
                text: "0: Create a 1/1 red Devil creature token with \"When \
                       this token dies, it deals 1 damage to any \
                       target.\"".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: You get an emblem with \"At the end of the first \
                       combat phase on your turn, untap target creature you \
                       control. After this phase, there is an additional \
                       combat phase.\"".into(),
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
                effect: minus_six,
            }),
    )
}

/// `+1`: +1/+0 anthem (haste-to-all rider not in the Anthem surface).
fn plus_one(_s: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP (partial): "and gain haste" — granting haste to all creatures
    // you control isn't part of Effect::Anthem (which carries P/T only).
    vec![Effect::Anthem {
        controller: ctx.controller,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
    }]
}

/// `0`: create the 1/1 red Devil token (death trigger rider omitted).
fn zero(_s: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    // GAP (partial): the token's "when this dies, deal 1 to any target"
    // triggered ability (a targeted death trigger) isn't built; the token
    // body is faithful.
    let devil = reg.interner().lookup("Devil").expect("Devil interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(devil);
    let token = TokenDefinition {
        name: devil,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

/// `−6`: bespoke combat-phase emblem.
fn minus_six(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: emblem with a combat-phase-end trigger + additional-combat-phase
    // rider is bespoke and not expressible from the demonstrated surface.
    Vec::new()
}
