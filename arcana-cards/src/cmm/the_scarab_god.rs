//! The Scarab God — `{3}{U}{B}` 5/5 Legendary God.
//! At the beginning of your upkeep, each opponent loses X life and you
//! scry X, where X = the number of Zombies you control.
//! `{2}{U}{B}: Exile target creature card from a graveyard. Create a
//! token that's a copy of it, except it's a 4/4 black Zombie.`
//! When The Scarab God dies, return it to its owner's hand at the
//! beginning of the next end step.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
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
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Scarab God");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: Scry (Scryfall-parsed) is a mechanic keyword, not a
        // KeywordAbility variant; the scry effect is wired in the upkeep
        // trigger below.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_drain_scry,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}{B}: Exile target creature card from a graveyard. Create a token that's a copy of it, except it's a 4/4 black Zombie."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exile_and_copy,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_return_delayed,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_drain_scry(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let zombies = script::subtype_filter(reg, "Zombie")
        .controlled_by(ControllerConstraint::You);
    let x = script::count_matching(state, &zombies, trig.controller);
    let mut effects: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::LoseLife {
            player: p,
            amount: x,
        })
        .collect();
    effects.push(Effect::Scry {
        player: trig.controller,
        count: x,
    });
    vec![Effect::Sequence(effects)]
}

fn exile_and_copy(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "create a token that's a copy of it, except it's a 4/4 black
    // Zombie" — Effect::CopyPermanent copies a battlefield permanent and
    // takes no characteristic-override; copying an exiled graveyard card
    // with stat/type overrides is not expressible. The exile half is
    // wired faithfully.
    vec![Effect::ExileFromGraveyard { target: *id }]
}

fn dies_return_delayed(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DelayedAction {
        source: trig.source,
        controller: trig.controller,
        when: DelayedWhen::NextEndStep,
        action: DelayedAction::ReturnToHand,
    }]
}
