//! Abhorrent Oculus — `{2}{U}` 5/5 blue Eye.
//!
//! * Additional cast cost "exile six cards from your graveyard" — not
//!   expressible (no additional-cast-cost field on this shape); GAP.
//! * Flying (keyword).
//! * "At the beginning of each opponent's upkeep, manifest dread." —
//!   best-effort with `Effect::Manifest`; manifest DREAD (look at top
//!   two, one face-down, the other to graveyard) is not a distinct
//!   primitive, so this is an over-simplification (GAP noted).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Abhorrent Oculus");
    let eye = reg.interner_mut().intern("Eye");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eye);
    // GAP: "As an additional cost to cast this spell, exile six cards
    // from your graveyard." — no additional-cast-cost on this shape.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: manifest_dread,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn manifest_dread(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: manifest DREAD (look at top two, one face-down 2/2, the other
    // to graveyard) is not a distinct primitive; plain Manifest is the
    // closest available effect (over-simplified).
    vec![Effect::Manifest { player: trig.controller }]
}
