//! Tezzeret, Artifice Master — `{3}{U}{U}` Legendary Planeswalker — Tezzeret, starting loyalty 5.
//!
//! +1: Create a 1/1 colorless Thopter artifact creature token with flying.
//! 0: Draw a card. If you control three or more artifacts, draw two cards instead.
//!    Modeled as a resolution-time count check on artifacts you control.
//! −9: You get an emblem with "At the beginning of your end step, search your
//!     library for a permanent card, put it onto the battlefield, then shuffle."
//!     Modeled as a Command-zone triggered emblem ability firing on your end step,
//!     resolving TutorToBattlefield for a permanent card.

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tezzeret, Artifice Master");
    let tezzeret = reg.interner_mut().intern("Tezzeret");
    let _thopter = reg.interner_mut().intern("Thopter");
    let _emblem = reg.interner_mut().intern("Tezzeret, Artifice Master emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tezzeret);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
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
                text: "+1: Create a 1/1 colorless Thopter artifact creature token \
                       with flying.".into(),
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
                effect: plus_one_thopter,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Draw a card. If you control three or more artifacts, \
                       draw two cards instead.".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-9: You get an emblem with \"At the beginning of your end \
                       step, search your library for a permanent card, put it onto \
                       the battlefield, then shuffle.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 9)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_nine_emblem,
            }),
    )
}

fn plus_one_thopter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let thopter = reg.interner().lookup("Thopter").expect("Thopter interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(thopter);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: thopter,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}

fn zero_draw(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let artifacts = ObjectFilter::new()
        .with_types(TypeLine::ARTIFACT.into())
        .controlled_by(ControllerConstraint::You);
    let count = if script::count_matching(state, &artifacts, ctx.controller) >= 3 {
        2
    } else {
        1
    };
    vec![Effect::DrawCards { player: ctx.controller, count }]
}

fn minus_nine_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Tezzeret, Artifice Master emblem")
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
                effect: emblem_tutor_permanent,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

fn emblem_tutor_permanent(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let permanent = ObjectFilter::new()
        .with_types_any(TypeLine(
            TypeLine::CREATURE
                | TypeLine::ARTIFACT
                | TypeLine::ENCHANTMENT
                | TypeLine::LAND
                | TypeLine::PLANESWALKER
                | TypeLine::BATTLE,
        ));
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: permanent,
        tapped: false,
    }]
}
