//! Ajani, Sleeper Agent — `{1}{G}{G/W/P}{W}` Legendary Planeswalker — Ajani.
//! Printed starting loyalty 5 (CR 113.3c). Colors G/W.
//!
//! Keyword: Compleated — NOT in the usable keyword surface (the
//! "enters with two fewer loyalty if life was paid" rider is unbuilt).
//! `keywords: vec![]`, GAP.
//!
//! Loyalty abilities (CR 606):
//! * `+1`: Reveal the top card of your library. If it's a creature or
//!   planeswalker card, put it into your hand; otherwise you may put it
//!   on the bottom. — GAP (reveal-and-conditional-route is not a
//!   demonstrated primitive; DigTopN's filtered take is the nearest but
//!   its "rest" handling and the reveal differ).
//! * `−3`: Distribute three +1/+1 counters among up to three target
//!   creatures. They gain vigilance until end of turn. — the vigilance
//!   grant is emitted per target; the "distribute three +1/+1 counters"
//!   half is GAP'd (no divided-counter primitive; only DealDamageDivided
//!   exists for damage).
//! * `−6`: You get an emblem with "Whenever you cast a creature or
//!   planeswalker spell, target opponent gets two poison counters." — GAP
//!   (emblem with a custom cast-trigger ability).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ajani, Sleeper Agent");
    let ajani = reg.interner_mut().intern("Ajani");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ajani);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G/W/P}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        // GAP: Compleated is not in the usable keyword surface.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Reveal the top card of your library. If it's a creature \
                       or planeswalker card, put it into your hand. Otherwise, you \
                       may put it on the bottom of your library.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_reveal,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Distribute three +1/+1 counters among up to three target \
                       creatures. They gain vigilance until end of turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(3),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_vigilance,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: You get an emblem with \"Whenever you cast a creature or \
                       planeswalker spell, target opponent gets two poison \
                       counters.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_emblem,
            }),
    )
}

/// `+1`: reveal-and-route — GAP.
fn plus_one_reveal(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reveal top; if creature/PW to hand, else may bottom" is a
    // reveal-and-conditional-route shape; no demonstrated primitive
    // expresses the reveal + the otherwise-may-bottom branch faithfully.
    Vec::new()
}

/// `−3`: grant vigilance to each target; the counter distribution is a GAP.
fn minus_three_vigilance(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "distribute three +1/+1 counters" has no divided-counter
    // primitive (DealDamageDivided is damage-only). The vigilance grant on
    // the chosen creatures is emitted.
    ctx.targets
        .targets
        .iter()
        .filter_map(|c| match c {
            TargetChoice::Object(id) => Some(Effect::GrantKeyword {
                target: *id,
                keyword: KeywordAbility::Vigilance,
                duration: Duration::EndOfTurn,
            }),
            _ => None,
        })
        .collect()
}

/// `−6`: emblem — GAP.
fn minus_six_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem carrying a custom cast-trigger poison ability — building
    // the EmblemDefinition's triggered ability is beyond the demonstrated
    // surface for this card.
    Vec::new()
}
