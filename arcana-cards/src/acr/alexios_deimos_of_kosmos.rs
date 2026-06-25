//! Alexios, Deimos of Kosmos — `{3}{R}` 4/4 legendary Human Berserker
//! (R) with Trample.
//!
//! * Trample
//! * Alexios attacks each combat if able (ETB-installed `must_attack`,
//!   CR 508.1a). "Can't be sacrificed" and "can't attack its owner"
//!   remain GAPs — no can't-be-sacrificed or per-defender can't-attack
//!   primitive.
//! * At the beginning of each player's upkeep, that player gains
//!   control of Alexios, untaps it, and puts a +1/+1 counter on it.
//!   It gains haste until end of turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Alexios, Deimos of Kosmos");
    let human = reg.interner_mut().intern("Human");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: upkeep_change_control,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
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

/// ETB: "Alexios attacks each combat if able." (CR 508.1a)
fn install_must_attack(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::must_attack(
            trig.source,
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn upkeep_change_control(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "that player" = the active player whose upkeep this is.
    let them = state.active_player();
    vec![
        Effect::ChangeControl { target: trig.source, new_controller: them },
        Effect::Untap { target: trig.source },
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ]
}
