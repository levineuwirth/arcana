//! Wrenn and One — Land Planeswalker — Wrenn, starting loyalty 3. Colors: G. No mana cost.
//! +1: Wrenn gains "{T}: Add {G}" until your next turn — GAP (no grant-activated-mana-ability primitive).
//! -1: Create a 1/1 green Squirrel creature token — implemented.
//! -4: emblem (begin precombat main: add {G} for each creature you control) — emblem trigger shell emitted; dynamic per-creature mana body GAP'd.

use arcana_core::effects::{Effect, EmblemDefinition, TokenDefinition};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wrenn and One");
    let sub = reg.interner_mut().intern("Wrenn");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let _emblem = reg.interner_mut().intern("Wrenn and One emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        colors: ColorSet::green(),
        types: (TypeLine::LAND | TypeLine::PLANESWALKER).into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    let _ = squirrel;

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Wrenn and One gains \"{T}: Add {G}\" until your next turn.".into(),
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
                effect: plus_one_grant,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Create a 1/1 green Squirrel creature token.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-4: You get an emblem with \"At the beginning of your precombat main \
                       phase, add {G} for each creature you control.\""
                    .into(),
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
                effect: minus_four_emblem,
            }),
    )
}

/// `+1`: grant Wrenn a "{T}: Add {G}" mana ability until your next turn.
fn plus_one_grant(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no primitive to grant a temporary activated mana ability to a permanent.
    Vec::new()
}

/// `-1`: create a 1/1 green Squirrel creature token.
fn minus_one_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let squirrel = reg.interner().lookup("Squirrel").expect("Squirrel interned");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(squirrel);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: squirrel,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

/// `-4`: triggered emblem — beginning of your precombat main phase.
fn minus_four_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Wrenn and One emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![],
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Main,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: emblem_precombat_main,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

/// Emblem trigger: add {G} for each creature you control.
fn emblem_precombat_main(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: dynamic "add {G} for each creature you control" — AddMana takes a fixed
    // Vec<ManaUnit>; the per-creature count is not expressible as a static amount.
    Vec::new()
}
