//! Ob Nixilis of the Black Oath — `{3}{B}{B}` Legendary Planeswalker — Nixilis,
//! starting loyalty 5. Mono-black. (Can be your commander — no rules effect.)
//!
//! +2: Each opponent loses 1 life; you gain life equal to the life lost this
//!   way. GAP: no "each opponent" iteration primitive and the life-gain is a
//!   dynamic sum of life lost. Ability shell declared with the +2 cost; effect
//!   GAP'd.
//! −2: Create a 5/5 black Demon creature token with flying. You lose 2 life.
//! −8: emblem ("{1}{B}, Sacrifice a creature: You gain X life and draw X cards,
//!   where X is the sacrificed creature's power."). GAP: an emblem with an
//!   ACTIVATED ability is not expressible — `EmblemDefinition` holds only statics
//!   and triggered abilities — and X is dynamic. Emblem shell created with no
//!   grant; effect GAP'd.

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ob Nixilis of the Black Oath");
    let nixilis = reg.interner_mut().intern("Nixilis");
    let _demon = reg.interner_mut().intern("Demon");
    let _emblem = reg.interner_mut().intern("Ob Nixilis of the Black Oath emblem");
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
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Create a 5/5 black Demon creature token with flying. \
                       You lose 2 life.".into(),
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
                effect: minus_two_demon,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: You get an emblem with \"{1}{B}, Sacrifice a \
                       creature: You gain X life and draw X cards, where X is \
                       the sacrificed creature's power.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_emblem,
            }),
    )
}

fn plus_two_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each opponent loses 1 life; you gain life equal to the life lost
    //      this way" — no each-opponent iteration / dynamic life-gain sum.
    Vec::new()
}

fn minus_two_demon(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let demon = reg.interner().lookup("Demon").expect("Demon interned");
    let mut st = SubtypeSet::default();
    st.0.insert(demon);
    vec![
        Effect::CreateToken {
            controller: ctx.controller,
            token: TokenDefinition {
                name: demon,
                colors: ColorSet::black(),
                types: TypeLine::CREATURE.into(),
                subtypes: st,
                power: Some(PtValue::Fixed(5)),
                toughness: Some(PtValue::Fixed(5)),
                keywords: vec![KeywordAbility::Flying],
                abilities: vec![],
            },
        },
        Effect::LoseLife { player: ctx.controller, amount: 2 },
    ]
}

fn minus_eight_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Ob Nixilis of the Black Oath emblem").expect("emblem interned");
    // GAP: the emblem grants an ACTIVATED ability with a dynamic-X payoff;
    //      EmblemDefinition holds only statics + triggered abilities.
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: Vec::new(),
        },
    }]
}
