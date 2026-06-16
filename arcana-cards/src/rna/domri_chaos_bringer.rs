//! Domri, Chaos Bringer — `{2}{R}{G}` Legendary Planeswalker — Domri, starting loyalty 4.
//!
//! +1: Add {R} or {G}; if spent on a creature spell it gains riot.
//!   Modeled as `AddMana` (one red mana as a stand-in for the R/G
//!   choice); the "riot if spent on a creature spell" rider is GAP'd.
//! −3: Look at the top four cards; reveal up to two creature cards and
//!   put them into your hand; bottom the rest. Modeled with `DigTopN`
//!   over a creature filter, rest to bottom random. (The fixed-count
//!   pick approximates "up to two".)
//! −8: You get an emblem with "At the beginning of each end step, create
//!   a 4/4 red and green Beast creature token with trample." Implemented
//!   as a triggered emblem firing on every end step.

use arcana_core::effects::{DigRest, Effect, EmblemDefinition, KeywordAbility, TokenDefinition};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::turn::Step;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Domri, Chaos Bringer");
    let domri = reg.interner_mut().intern("Domri");
    let _beast = reg.interner_mut().intern("Beast");
    let _emblem = reg.interner_mut().intern("Domri, Chaos Bringer emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(domri);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Add {R} or {G}. If that mana is spent on a \
                       creature spell, it gains riot.".into(),
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
                effect: plus_one_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Look at the top four cards of your library. You \
                       may reveal up to two creature cards from among them \
                       and put them into your hand. Put the rest on the \
                       bottom of your library in a random order.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: You get an emblem with \"At the beginning of each \
                       end step, create a 4/4 red and green Beast creature \
                       token with trample.\"".into(),
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

fn plus_one_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the R/G choice and the "spent on a creature spell → riot"
    // rider aren't expressible; add one red mana as a stand-in.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
    }]
}

fn minus_three_dig(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 4,
        filter: Some(ObjectFilter {
            types: Some(TypeLine::CREATURE.into()),
            ..Default::default()
        }),
        rest: DigRest::BottomRandom,
    }]
}

fn minus_eight_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Domri, Chaos Bringer emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: emblem_make_beast,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

fn emblem_make_beast(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let beast = reg.interner().lookup("Beast").expect("Beast interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(beast);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: beast,
            colors: ColorSet::red() | ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Trample],
            abilities: vec![],
        },
    }]
}
