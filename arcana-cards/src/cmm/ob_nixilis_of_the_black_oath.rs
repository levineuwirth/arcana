//! Ob Nixilis of the Black Oath — `{3}{B}{B}` legendary planeswalker, starting loyalty 5.
//! Mono-black planeswalker (subtype Nixilis).
//!
//! # Rules references
//!
//! * CR 606 — loyalty abilities; CR 606.3 — sorcery-speed, controller-only,
//!   once per turn per planeswalker. CR 704.5i — 0-loyalty sacrifice SBA.
//!
//! # Scope
//!
//! * `+2`: Each opponent loses 1 life; you gain life equal to the life lost
//!   this way. Expressed by losing 1 per opponent and gaining a number of
//!   life equal to the opponent count (each loses exactly 1).
//! * `−2`: Create a 5/5 black Demon creature token with flying; you lose 2
//!   life — fully expressed.
//! * `−8`: Emblem — GAP (emblem creation is not in the demonstrated surface).
//!
//! "Ob Nixilis of the Black Oath can be your commander" is a deck-construction
//! rule, not a loyalty ability; not modeled.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ob Nixilis of the Black Oath");
    let nixilis = reg.interner_mut().intern("Nixilis");
    let _demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nixilis);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Each opponent loses 1 life. You gain life equal to \
                       the life lost this way.".into(),
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
                effect: plus_two_drain,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Create a 5/5 black Demon creature token with flying. \
                       You lose 2 life.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_demon,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: You get an emblem with \"{1}{B}, Sacrifice a \
                       creature: You gain X life and draw X cards, where X is \
                       the sacrificed creature's power.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_emblem,
            }),
    )
}

/// `+2: Each opponent loses 1 life. You gain life equal to the life lost.`
fn plus_two_drain(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opps = script::opponents(state, ctx.controller);
    let mut out: Vec<Effect> = Vec::new();
    for opp in &opps {
        out.push(Effect::LoseLife { player: *opp, amount: 1 });
    }
    // Each opponent loses exactly 1, so total life lost this way == count.
    out.push(Effect::GainLife { player: ctx.controller, amount: opps.len() as u32 });
    out
}

/// `−2: Create a 5/5 black Demon creature token with flying. You lose 2 life.`
fn minus_two_demon(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let demon = reg.interner().lookup("Demon").expect("Demon interned at register");
    let mut demon_subtypes = SubtypeSet::default();
    demon_subtypes.0.insert(demon);
    let token = TokenDefinition {
        name: demon,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: demon_subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: ctx.controller, token },
        Effect::LoseLife { player: ctx.controller, amount: 2 },
    ]
}

/// `−8: You get an emblem with the sacrifice-for-life-and-cards ability.`
fn minus_eight_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem creation is not in the demonstrated effect surface.
    Vec::new()
}
