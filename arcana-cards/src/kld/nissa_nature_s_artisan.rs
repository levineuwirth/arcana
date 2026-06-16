//! Nissa, Nature's Artisan — `{4}{G}{G}` Legendary Planeswalker — Nissa.
//! Starting loyalty 5 (oracle).
//!
//! +3: You gain 3 life.
//! −4: Reveal the top two cards of your library. Put all land cards from
//!     among them onto the battlefield and the rest into your hand.
//!     GAP: "reveal top N, put ALL matching onto battlefield, rest into hand"
//!     (a multi-card split by type) is not expressible.
//! −12: Creatures you control get +5/+5 and gain trample until end of turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nissa, Nature's Artisan");
    let nissa = reg.interner_mut().intern("Nissa");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nissa);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
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
                text: "+3: You gain 3 life.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_three_life,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-4: Reveal the top two cards of your library. Put all \
                       land cards onto the battlefield and the rest into your \
                       hand.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_four_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-12: Creatures you control get +5/+5 and gain trample \
                       until end of turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 12)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_twelve_anthem,
            }),
    )
}

fn plus_three_life(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife {
        player: ctx.controller,
        amount: 3,
    }]
}

fn minus_four_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reveal-top-2, put all lands to battlefield, rest to hand — a
    //      multi-card by-type split is not expressible.
    Vec::new()
}

fn minus_twelve_anthem(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // +5/+5 to your creatures this turn, plus trample granted per creature.
    let your_creatures = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &your_creatures, ctx.controller);
    let mut effects = vec![Effect::Anthem {
        controller: ctx.controller,
        power: 5,
        toughness: 5,
        duration: Duration::EndOfTurn,
    }];
    for id in ids {
        effects.push(Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        });
    }
    effects
}
