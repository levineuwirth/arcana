//! Gogo, Mysterious Mime — `{3}{R}` 2/2 red Legendary Wizard.
//! "At the beginning of combat on your turn, you may have Gogo become a copy of another
//! target creature you control until end of turn, except its name is Gogo, Mysterious Mime.
//! If you do, Gogo and that creature each get +2/+0 and gain haste until end of turn and
//! attack this turn if able."
//! GAP: "become a copy of another creature (name exception)" requires CopyPermanent-self
//! variant not in catalog; effect returns Vec::new() for the copy step.
//! Partial: best-effort +2/+0, Haste, and an EndOfTurn must-attack on both
//! Gogo and the target creature (the copy step itself remains GAP'd).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gogo, Mysterious Mime");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
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
                intervening_if: None,
                effect: combat_copy_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: arcana_core::targets::TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: arcana_core::targets::TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn combat_copy_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    use arcana_core::targets::TargetChoice;
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "Gogo becomes a copy of target creature (except name)" not in engine catalog.
    // Best-effort: pump both Gogo (trig.source) and the target creature +2/+0 + Haste,
    // and force both to attack this turn if able.
    vec![
        Effect::Pump {
            target: trig.source,
            power: 2,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Haste],
        },
        Effect::Pump {
            target: *id,
            power: 2,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Haste],
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::must_attack(trig.source, trig.source, Duration::EndOfTurn),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::must_attack(trig.source, *id, Duration::EndOfTurn),
        },
    ]
}
