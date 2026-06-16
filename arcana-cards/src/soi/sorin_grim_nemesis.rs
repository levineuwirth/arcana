//! Sorin, Grim Nemesis — `{4}{W}{B}` Legendary Planeswalker — Sorin,
//! starting loyalty 6.
//!
//! * `+1`: Reveal the top card of your library and put it into your hand;
//!   each opponent loses life equal to its mana value. GAP'd (top-reveal-
//!   to-hand + dynamic per-opponent mana-value life loss).
//! * `−X`: Sorin deals X damage to target creature or planeswalker and you
//!   gain X life. OMITTED — dynamic-X loyalty cost is not expressible
//!   (`remove_self_counter` is a fixed `u32`).
//! * `−9`: Create a number of 1/1 black Vampire Knight tokens with lifelink
//!   equal to the highest life total among all players. GAP'd (dynamic
//!   token count from a resolution-time amount).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sorin, Grim Nemesis");
    let sorin = reg.interner_mut().intern("Sorin");
    let _vampire = reg.interner_mut().intern("Vampire");
    let _knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sorin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(6),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Reveal the top card of your library and put that card \
                       into your hand. Each opponent loses life equal to its \
                       mana value.".into(),
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
                effect: plus_one_reveal,
            })
            // GAP: "−X: Sorin deals X damage … you gain X life" — dynamic-X
            // loyalty cost is not expressible; the ability is omitted.
            .with_activated_ability(ActivatedAbilityDef {
                text: "−9: Create a number of 1/1 black Vampire Knight creature \
                       tokens with lifelink equal to the highest life total \
                       among all players.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 9)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_nine_vampires,
            }),
    )
}

fn plus_one_reveal(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reveal-top-to-hand + dynamic per-opponent mana-value life loss.
    Vec::new()
}

fn minus_nine_vampires(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: dynamic token count (= highest life total among all players).
    Vec::new()
}
