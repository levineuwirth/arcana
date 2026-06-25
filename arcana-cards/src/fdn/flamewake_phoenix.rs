//! Flamewake Phoenix — `{1}{R}{R}` 2/2 Phoenix with Flying, Haste.
//! "Attacks each combat if able." Ferocious — at the beginning of combat on
//! your turn, if you control a creature with power 4+, you may pay {R} to
//! return this card from your graveyard to the battlefield.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flamewake Phoenix");
    let phoenix = reg.interner_mut().intern("Phoenix");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phoenix);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_control_power_four),
                effect: maybe_return_from_graveyard,
                trigger_zones: vec![Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "Attacks each combat if able" — SELF must-attack installed on ETB.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_must_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn install_must_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::must_attack(
            trig.source,
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn if_control_power_four(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::you_control_at_least(
        s,
        you,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .with_min_power(4),
        1,
    )
}

fn maybe_return_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{R}").expect("valid cost")),
        then: Box::new(Effect::ReturnFromGraveyardToBattlefield { target: trig.source }),
        else_effect: None,
    }]
}
