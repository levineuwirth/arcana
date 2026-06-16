//! Garruk, Unleashed — `{2}{G}{G}` Legendary Planeswalker — Garruk, starting loyalty 3.
//!
//! +1: Up to one target creature gets +3/+3 and gains trample until end
//!   of turn.
//! −2: Create a 3/3 green Beast creature token. Then if an opponent
//!   controls more creatures than you, put a loyalty counter on Garruk.
//!   The conditional self-loyalty bump is computed at resolution via
//!   `script::count_matching` (your creatures vs. the opponent's).
//! −7: You get an emblem with "At the beginning of your end step, you
//!   may search your library for a creature card, put it onto the
//!   battlefield, then shuffle." Implemented as a triggered emblem with
//!   `Effect::Search` (battlefield destination, shuffle implied).

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::turn::Step;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Garruk, Unleashed");
    let garruk = reg.interner_mut().intern("Garruk");
    let _beast = reg.interner_mut().intern("Beast");
    let _emblem = reg.interner_mut().intern("Garruk, Unleashed emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(garruk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Up to one target creature gets +3/+3 and gains \
                       trample until end of turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_pump,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Create a 3/3 green Beast creature token. Then if \
                       an opponent controls more creatures than you, put a \
                       loyalty counter on Garruk.".into(),
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
                effect: minus_two_beast,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: You get an emblem with \"At the beginning of your \
                       end step, you may search your library for a creature \
                       card, put it onto the battlefield, then shuffle.\"".into(),
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

fn plus_one_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 3,
        toughness: 3,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Trample],
    }]
}

fn minus_two_beast(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let beast = reg.interner().lookup("Beast").expect("Beast interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(beast);
    let mut effects = vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: beast,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            abilities: vec![],
        },
    }];

    let mine = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    let theirs = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
        ctx.controller,
    );
    if theirs > mine {
        effects.push(Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Loyalty,
            count: 1,
        });
    }
    effects
}

fn minus_seven_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Garruk, Unleashed emblem")
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
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: emblem_search_creature,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

fn emblem_search_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Search {
        player: trig.controller,
        zone: Zone::Library(trig.controller),
        filter: ObjectFilter {
            types: Some(TypeLine::CREATURE.into()),
            ..Default::default()
        },
        destination: Zone::Battlefield,
        reveal: false,
    }]
}
