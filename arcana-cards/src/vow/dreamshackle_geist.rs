//! Dreamshackle Geist — `{1}{U}{U}` 3/1 Spirit with Flying.
//!
//! Oracle:
//! * Flying.
//! * At the beginning of combat on your turn, choose up to one —
//!   • Tap target creature.
//!   • Target creature doesn't untap during its controller's next untap step.
//!
//! Flying is a base characteristic. Triggered abilities have no modal field, so
//! the "choose up to one" structure isn't expressible. We implement the first
//! mode faithfully (tap target creature) as a single-target combat trigger and
//! GAP the modal choice + the second mode (no "doesn't untap next untap step"
//! Effect exists).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dreamshackle Geist");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // At the beginning of combat on your turn, choose up to one —
            // (modal not expressible on a triggered ability; mode 0 "Tap target
            // creature" implemented, mode 1 GAP'd).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: tap_target_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
    // GAP: "choose up to one" modality is not expressible on a triggered
    // ability (no modal field on TriggeredAbilityDef).
    // GAP: mode 2 "Target creature doesn't untap during its controller's next
    // untap step" — no skip-untap Effect.
}

fn tap_target_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Tap { target: *id }]
}
