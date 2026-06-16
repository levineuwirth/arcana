//! Teferi, Temporal Archmage — `{4}{U}{U}` Legendary Planeswalker — Teferi, starting loyalty 5.
//!
//! +1: Look at the top two cards of your library. Put one of them into your hand
//!   and the other on the bottom of your library. Modeled with `Effect::DigTopN`
//!   (count 2, pick one to hand, rest to bottom).
//! −1: Untap up to four target permanents.
//! −10: You get an emblem with "You may activate loyalty abilities of
//!   planeswalkers you control on any player's turn any time you could cast an
//!   instant." GAP: an instant-speed-loyalty permission emblem is not expressible;
//!   shell declared at the correct −10 cost.
//! ("Teferi, Temporal Archmage can be your commander." — commander designation;
//!   not modeled as a card ability.)

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teferi, Temporal Archmage");
    let teferi = reg.interner_mut().intern("Teferi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(teferi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Look at the top two cards of your library. Put one of \
                       them into your hand and the other on the bottom of your \
                       library.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Untap up to four target permanents.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::new()),
                    count: TargetCount::UpTo(4),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_untap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-10: You get an emblem with \"You may activate loyalty \
                       abilities of planeswalkers you control on any player's turn \
                       any time you could cast an instant.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 10)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_ten_gap,
            }),
    )
}

fn plus_one_dig(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 2,
        filter: None,
        rest: DigRest::BottomRandom,
    }]
}

fn minus_one_untap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for t in &ctx.targets.targets {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::Untap { target: *id });
        }
    }
    effects
}

fn minus_ten_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: instant-speed loyalty-activation permission emblem not expressible.
    Vec::new()
}
