//! Kellan, Planar Trailblazer — `{R}` 2/1 Legendary Human Faerie Scout.
//! 1. "{1}{R}: If Kellan is a Scout, it becomes a Human Faerie Detective
//!    and gains 'Whenever Kellan deals combat damage to a player, exile
//!    the top card of your library. You may play that card this turn.'"
//!    (The granted combat-damage impulse-draw trigger is wired.
//!    GAP: the "if Kellan is a Scout" precondition and the creature-
//!    subtype change to Detective are not expressible.)
//! 2. "{2}{R}: If Kellan is a Detective, it becomes a 3/2 Human Faerie
//!    Rogue and gains double strike." (The 3/2 base P/T and double
//!    strike grant are wired. GAP: the "if Detective" precondition and
//!    the subtype change to Rogue are not expressible.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kellan, Planar Trailblazer");
    let human = reg.interner_mut().intern("Human");
    let faerie = reg.interner_mut().intern("Faerie");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(faerie);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: If Kellan is a Scout, it becomes a Human Faerie Detective and gains \"Whenever Kellan deals combat damage to a player, exile the top card of your library. You may play that card this turn.\"".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_detective,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}: If Kellan is a Detective, it becomes a 3/2 Human Faerie Rogue and gains double strike.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_rogue,
            }),
    )
}

fn become_detective(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if Kellan is a Scout" precondition and the subtype change to
    // Human Faerie Detective are not expressible. The granted combat-
    // damage impulse-draw triggered ability is wired.
    vec![Effect::GrantTriggeredAbility {
        target: ctx.source,
        ability: Box::new(TriggeredAbilityDef {
            id: GRANTED_TRIGGER_ID_BASE + 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: arcana_core::targets::ObjectFilter::permanent(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: impulse_one,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
        duration: Duration::WhileSourceOnBattlefield,
    }]
}

fn impulse_one(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::ImpulseExile {
        player: trig.controller,
        count: 1,
    }]
}

fn become_rogue(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "if Kellan is a Detective" precondition and the subtype change
    // to Human Faerie Rogue are not expressible.
    vec![
        Effect::SetBasePT {
            target: ctx.source,
            power: 3,
            toughness: 2,
            duration: Duration::Permanent,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::DoubleStrike,
            duration: Duration::Permanent,
        },
    ]
}
