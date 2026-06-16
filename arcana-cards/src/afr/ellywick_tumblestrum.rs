//! Ellywick Tumblestrum — `{2}{G}{G}` Legendary Planeswalker — Ellywick,
//! starting loyalty 5. Colors G.
//!
//! +1: Venture into the dungeon (`Effect::Venture`).
//! −2: Look at the top six cards of your library; you may reveal a creature card
//!   and put it into your hand; rest on the bottom in a random order. Modeled
//!   with `Effect::DigTopN { count: 6, filter: creature, rest: BottomRandom }`.
//!   GAP: "if it's legendary, you gain 3 life" rider (no conditional hook on the
//!   chosen card).
//! −7: emblem ("Creatures you control have trample and haste and get +2/+2 for
//!   each differently named dungeon you've completed."). Static emblem: trample
//!   and haste keyword anthems are installed; GAP: the dynamic "+2/+2 for each
//!   differently named dungeon completed" pump is not expressible (dynamic-X).

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility, DigRest};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ellywick Tumblestrum");
    let ellywick = reg.interner_mut().intern("Ellywick");
    let _emblem = reg.interner_mut().intern("Ellywick Tumblestrum emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ellywick);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
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
                text: "-2: Look at the top six cards of your library. You may \
                       reveal a creature card from among them and put it into \
                       your hand. If it's legendary, you gain 3 life. Put the \
                       rest on the bottom of your library in a random \
                       order.".into(),
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
                text: "-7: You get an emblem with \"Creatures you control have \
                       trample and haste and get +2/+2 for each differently \
                       named dungeon you've completed.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_emblem,
            }),
    )
}

fn plus_one_venture(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Venture { player: ctx.controller }]
}

fn minus_two_dig(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if it's legendary, you gain 3 life" rider on the chosen card.
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 6,
        filter: Some(ObjectFilter::new().with_types(TypeLine::CREATURE.into())),
        rest: DigRest::BottomRandom,
    }]
}

fn minus_seven_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Ellywick Tumblestrum emblem").expect("emblem interned");
    // GAP: "+2/+2 for each differently named dungeon you've completed" (dynamic-X
    //      anthem). The trample and haste keyword anthems ARE installed.
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![
                ContinuousEffect::keyword_anthem(
                    NULL_OBJECT_ID, ctx.controller, KeywordAbility::Trample,
                    Duration::Permanent,
                ),
                ContinuousEffect::keyword_anthem(
                    NULL_OBJECT_ID, ctx.controller, KeywordAbility::Haste,
                    Duration::Permanent,
                ),
            ],
            abilities: Vec::new(),
        },
    }]
}
