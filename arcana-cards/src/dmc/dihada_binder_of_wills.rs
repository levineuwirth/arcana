//! Dihada, Binder of Wills — `{1}{R}{W}{B}` Legendary Planeswalker — Dihada,
//! starting loyalty 5.
//! +2: Up to one target legendary creature gains vigilance, lifelink, and
//!   indestructible until your next turn.
//! −3: Reveal the top four cards of your library. Put any number of legendary
//!   cards from among them into your hand and the rest into your graveyard.
//!   Create a Treasure token for each card put into your graveyard this way.
//! −11: Gain control of all nonland permanents until end of turn. Untap them.
//!   They gain haste until end of turn.
//! Dihada, Binder of Wills can be your commander.
//!
//! GAP: +2 uses an "until your next turn" duration and an "up to one target"
//!   shape not expressible with the demonstrated Pump/GrantKeyword surface.
//! GAP: −3 reveal-and-sort-by-legendary + per-card Treasure creation is a
//!   bespoke effect not in the demonstrated Effect catalog.
//! GAP: −11 mass control-change + untap + haste is a bespoke effect not in
//!   the demonstrated Effect catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dihada, Binder of Wills");
    let dihada = reg.interner_mut().intern("Dihada");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dihada);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // +2: GAP (until-your-next-turn duration, up-to-one target)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Up to one target legendary creature gains vigilance, lifelink, and indestructible until your next turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_gap,
            })
            // −3: GAP (reveal-sort + per-card Treasure)
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Reveal the top four cards of your library. Put any number of legendary cards from among them into your hand and the rest into your graveyard. Create a Treasure token for each card put into your graveyard this way.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_gap,
            })
            // −11: GAP (mass control-change + untap + haste)
            .with_activated_ability(ActivatedAbilityDef {
                text: "-11: Gain control of all nonland permanents until end of turn. Untap them. They gain haste until end of turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 11)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eleven_gap,
            }),
    )
}

fn plus_two_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "until your next turn" duration + "up to one target" not expressible.
    Vec::new()
}

fn minus_three_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reveal-and-sort-by-legendary + per-card Treasure creation not in catalog.
    Vec::new()
}

fn minus_eleven_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: mass control-change + untap + haste not in the demonstrated catalog.
    Vec::new()
}
