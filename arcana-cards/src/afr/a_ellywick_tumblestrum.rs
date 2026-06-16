//! A-Ellywick Tumblestrum — `{2}{G}{G}` Legendary Planeswalker — Ellywick, starting loyalty 4.
//!
//! +1: Venture into the dungeon. (GAP — venture/dungeon-advance is not part of
//!     the demonstrated Effect surface for this card class.)
//! −2: Look at the top six cards of your library, reveal a creature card, put
//!     it into your hand; if legendary gain 3 life; rest to bottom in random
//!     order. (GAP — dig-and-reveal-from-library is not expressible here.)
//! −6: You get an emblem with "Creatures you control have trample and haste and
//!     get +2/+2 for each differently named dungeon you've completed." The
//!     keyword grants (trample + haste) are implemented as keyword anthems; the
//!     dynamic "+2/+2 for each differently named dungeon" pump GAPs (no
//!     dungeon-count accessor in the demonstrated surface).

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Ellywick Tumblestrum");
    let ellywick = reg.interner_mut().intern("Ellywick");
    let _emblem = reg.interner_mut().intern("A-Ellywick Tumblestrum emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ellywick);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Venture into the dungeon.".into(),
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
                effect: plus_one_venture,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Look at the top six cards of your library. You may \
                       reveal a creature card from among them and put it into \
                       your hand. If it's legendary, you gain 3 life. Put the \
                       rest on the bottom of your library in a random order."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: You get an emblem with \"Creatures you control have \
                       trample and haste and get +2/+2 for each differently \
                       named dungeon you've completed.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_emblem,
            }),
    )
}

/// `+1: Venture into the dungeon.`
fn plus_one_venture(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: venture-into-the-dungeon / dungeon-advance not in the demonstrated surface.
    Vec::new()
}

/// `−2: dig six, reveal a creature, gain 3 if legendary, rest to bottom.`
fn minus_two_dig(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: look-at-top-N / reveal-from-library / bottom-in-random-order not expressible here.
    Vec::new()
}

/// `−6: emblem — trample + haste anthems; dynamic dungeon pump GAP'd.`
fn minus_six_emblem(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("A-Ellywick Tumblestrum emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            // Trample + haste for your creatures. The "+2/+2 for each
            // differently named dungeon you've completed" dynamic pump GAPs
            // (no dungeon-count accessor).
            statics: vec![
                ContinuousEffect::keyword_anthem(
                    NULL_OBJECT_ID,
                    ctx.controller,
                    KeywordAbility::Trample,
                    Duration::Permanent,
                ),
                ContinuousEffect::keyword_anthem(
                    NULL_OBJECT_ID,
                    ctx.controller,
                    KeywordAbility::Haste,
                    Duration::Permanent,
                ),
            ],
            abilities: vec![],
        },
    }]
}
