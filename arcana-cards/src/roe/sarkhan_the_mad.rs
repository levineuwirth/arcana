//! Sarkhan the Mad — `{3}{B}{R}` Legendary Planeswalker — Sarkhan,
//! starting loyalty 7.
//!
//! 0: Reveal the top card of your library and put it into your hand. Sarkhan
//!   deals damage to himself equal to that card's mana value.
//! −2: Target creature's controller sacrifices it, then that player creates a
//!   5/5 red Dragon creature token with flying.
//! −4: Each Dragon creature you control deals damage equal to its power to
//!   target player or planeswalker.
//!
//! GAP: the 0 ability's "Sarkhan deals damage to himself equal to that card's
//!   mana value" is a dynamic-X self-damage tied to a just-revealed card; not
//!   expressible in the demonstrated Effect surface. The reveal-to-hand half
//!   is also a reveal-the-top-card-then-hand effect with no demonstrated
//!   variant, so the whole ability is GAP'd.
//! GAP: −2 "target creature's controller sacrifices it, then THAT PLAYER
//!   creates a 5/5 red Dragon" — the token must be created under the target's
//!   controller (not the activator), which the ActivationContext surface
//!   can't address; ability shell declared with correct cost, effect GAP'd.
//! GAP: −4 "each Dragon creature you control deals damage equal to ITS POWER"
//!   is a per-creature dynamic-power damage fan-out; not expressible. Ability
//!   shell declared with correct cost, effect GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sarkhan the Mad");
    let sub = reg.interner_mut().intern("Sarkhan");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(7),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // 0: reveal top to hand, deal MV damage to self (GAP)
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Reveal the top card of your library and put it into your hand. Sarkhan the Mad deals damage to himself equal to that card's mana value.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_reveal_selfdamage,
            })
            // −2: target creature's controller sacrifices it, makes a Dragon (GAP)
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Target creature's controller sacrifices it, then that player creates a 5/5 red Dragon creature token with flying.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_sac_dragon,
            })
            // −4: each Dragon you control deals its power to target player/PW (GAP)
            .with_activated_ability(ActivatedAbilityDef {
                text: "-4: Each Dragon creature you control deals damage equal to its power to target player or planeswalker.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_four_dragon_burn,
            }),
    )
}

fn zero_reveal_selfdamage(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reveal-top-to-hand + dynamic self-damage equal to that card's
    // mana value — neither half is expressible in the demonstrated surface.
    Vec::new()
}

fn minus_two_sac_dragon(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the 5/5 Dragon token must be created under the TARGET's
    // controller, not the activator; not addressable here.
    Vec::new()
}

fn minus_four_dragon_burn(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: per-Dragon dynamic-power damage fan-out to a player-or-planeswalker
    // target; not expressible.
    Vec::new()
}
