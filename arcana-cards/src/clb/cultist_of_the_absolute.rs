//! Cultist of the Absolute — `{B}` Legendary Enchantment — Background.
//! "Commander creatures you own get +3/+3 and have flying, deathtouch,
//! 'Ward—Pay 3 life,' and 'At the beginning of your upkeep, sacrifice
//! a creature.'"
//!
//! Implementation: ETB installs, on commander creatures you own
//! ('you own' approximated by `controlled_by(ControllerConstraint::You)`),
//! a commander-filtered +3/+3 pump, flying + deathtouch keyword grants,
//! and a granted upkeep "sacrifice a creature" triggered ability — all
//! with `Duration::WhileSourceOnBattlefield`. The non-mana ward
//! ("Ward—Pay 3 life") is NOT expressible (see GAP below).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cultist of the Absolute");
    let background = reg.interner_mut().intern("Background");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(background);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_grants,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// The "Commander creatures you own" filter.
fn commander_filter() -> ObjectFilter {
    ObjectFilter::creature()
        .commander_only()
        .controlled_by(ControllerConstraint::You)
}

/// ETB: install the +3/+3 pump, the flying and deathtouch grants, and
/// the granted upkeep-sacrifice triggered ability.
fn etb_install_grants(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the granted "Ward—Pay 3 life" is a non-mana ward cost,
    // which is not expressible (KeywordAbility::Ward only takes a
    // ManaCost).
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_pump(
                trig.source,
                commander_filter(),
                3,
                3,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                commander_filter(),
                KeywordAbility::Flying,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                commander_filter(),
                KeywordAbility::Deathtouch,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_grant_triggered(
                trig.source,
                commander_filter(),
                TriggeredAbilityDef {
                    id: arcana_core::triggers::GRANTED_TRIGGER_ID_BASE + 1,
                    trigger_condition: TriggerCondition::StepBegins {
                        step: Step::Upkeep,
                        whose: ControllerConstraint::You,
                    },
                    intervening_if: None,
                    effect: granted_upkeep_sacrifice,
                    trigger_zones: vec![Zone::Battlefield],
                    frequency: TriggerFrequency::EachTime,
                    target_requirements: Vec::new(),
                },
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}

/// Granted ability: at the beginning of your upkeep, sacrifice a
/// creature.
fn granted_upkeep_sacrifice(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        count: 1,
    }]
}
