//! Belligerent Yearling — `{1}{R}` 3/2 Dinosaur with Trample.
//! "Whenever another Dinosaur you control enters, you may have this
//! creature's base power become equal to that creature's power until
//! end of turn."
//!
//! Trample is an engine keyword. The ETB trigger watches Dinosaurs you
//! control entering and sets this creature's base power to the entering
//! Dinosaur's power until end of turn. `SetBasePT` sets both P/T, so
//! toughness is re-stated at its current value to leave it effectively
//! unchanged. The "another" self-exclusion is a minor documented
//! over-fire (its own ETB would also trigger), and the "you may" choice
//! is applied unconditionally as a best-effort.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Belligerent Yearling");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let dino_filter =
        script::subtype_filter(reg, "Dinosaur").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: dino_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: set_base_power_to_entering,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn set_base_power_to_entering(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let entered = trig.entering_object().unwrap_or(trig.source);
    if entered == trig.source {
        return Vec::new();
    }
    let new_power = script::power_of(state, entered);
    let cur_toughness = script::toughness_of(state, trig.source);
    vec![Effect::SetBasePT {
        target: trig.source,
        power: new_power,
        toughness: cur_toughness,
        duration: Duration::EndOfTurn,
    }]
}
