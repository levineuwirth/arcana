//! Popular Egotist — `{2}{B}` 3/2 black Human Rogue.
//! "{1}{B}, Sacrifice another creature or enchantment: This creature gains
//! indestructible until end of turn. Tap it."
//! "Whenever you sacrifice a permanent, target opponent loses 1 life and you
//! gain 1 life."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Popular Egotist");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}, Sacrifice another creature or enchantment: This creature gains indestructible until end of turn. Tap it.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                    sacrifice_other: Some(
                        ObjectFilter::permanent()
                            .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::ENCHANTMENT)),
                    ),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_indestructible_and_tap,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::Sacrificed {
                    filter: ObjectFilter::permanent().controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: drain_on_sacrifice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // "target opponent" — engine player target; opponent restriction
                // is not separately encodable, using a player target.
                target_requirements: vec![TargetRequirement::target_player()],
            }),
    )
}

fn gain_indestructible_and_tap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        },
        Effect::Tap { target: ctx.source },
    ]
}

fn drain_on_sacrifice(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![
        Effect::LoseLife {
            player: *p,
            amount: 1,
        },
        Effect::GainLife {
            player: trig.controller,
            amount: 1,
        },
    ]
}
