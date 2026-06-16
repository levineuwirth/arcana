//! Teyo, Aegis Adept — `{2}{W}{W}` Legendary Planeswalker — Teyo, starting
//! loyalty 5. Mono-white.
//!
//! +1: Up to one target creature's base power perpetually becomes its toughness;
//!   it perpetually gains "can attack as though it didn't have defender." GAP:
//!   perpetual base-P/T modification and the perpetual defender-override are not
//!   expressible. Ability shell declared with the +1 cost; effect GAP'd.
//! −2: Conjure a card named Lumbering Lightshield onto the battlefield. GAP: no
//!   Conjure effect in the demonstrated surface. Ability shell declared.
//! −6: emblem ("At the beginning of your end step, return target white creature
//!   card from your graveyard to the battlefield. You gain life equal to its
//!   toughness."). Triggered emblem: StepBegins(End, You), targets a white
//!   creature card in your graveyard → ReturnFromGraveyardToBattlefield. GAP: the
//!   "gain life equal to its toughness" rider is dynamic.

use arcana_core::effects::{Effect, EmblemDefinition};
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
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teyo, Aegis Adept");
    let teyo = reg.interner_mut().intern("Teyo");
    let _emblem = reg.interner_mut().intern("Teyo, Aegis Adept emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(teyo);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Up to one target creature's base power perpetually \
                       becomes equal to its toughness. It perpetually gains \
                       \"This creature can attack as though it didn't have \
                       defender.\"".into(),
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
                effect: plus_one_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Conjure a card named Lumbering Lightshield onto the \
                       battlefield.".into(),
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
                effect: minus_two_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: You get an emblem with \"At the beginning of your \
                       end step, return target white creature card from your \
                       graveyard to the battlefield. You gain life equal to its \
                       toughness.\"".into(),
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

fn plus_one_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: perpetual base-power-becomes-toughness and the perpetual
    //      defender-override are not expressible.
    Vec::new()
}

fn minus_two_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Conjure (create a card by name onto the battlefield) is not in the
    //      demonstrated Effect surface.
    Vec::new()
}

fn minus_six_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Teyo, Aegis Adept emblem").expect("emblem interned");
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
                effect: emblem_reanimate,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(ctx.controller),
                        filter: ObjectFilter::creature().with_colors(ColorSet::white()),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }],
        },
    }]
}

fn emblem_reanimate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else { return Vec::new(); };
    // GAP: "you gain life equal to its toughness" — dynamic amount.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
