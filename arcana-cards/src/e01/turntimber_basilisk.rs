//! Turntimber Basilisk — `{1}{G}{G}` 2/1 green Basilisk.
//!
//! * Deathtouch (CR 702.2) — base keyword on this card.
//! * Landfall — "Whenever a land you control enters, you may have target
//!   creature block this creature this turn if able." Modeled as a single
//!   targeted `ZoneChange` trigger (land entering the battlefield under your
//!   control). The forced-block ("must block this creature this turn if
//!   able") payload has no matching `Effect` variant in the catalog, so the
//!   effect fn GAPs the body while the trigger + target requirement are
//!   faithfully wired.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Turntimber Basilisk");
    let basilisk = reg.interner_mut().intern("Basilisk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(basilisk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: landfall_force_block,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

/// Landfall: "you may have target creature block this creature this turn if
/// able."
fn landfall_force_block(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no Effect variant for "must block this creature this turn if able" (forced-block/Lure)
    Vec::new()
}
