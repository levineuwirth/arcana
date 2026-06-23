//! Fearless Swashbuckler — `{1}{U}{R}` 3/3 Fish Pirate with Haste.
//!
//! * Haste.
//! * Vehicles you control have haste. (static — NOW WIRED via an ETB-installed
//!   `ContinuousEffect::filtered_keyword` over Vehicles you control.)
//! * Whenever you attack, if a Pirate and a Vehicle attacked this combat, draw
//!   three cards, then discard two cards. (GAP — no "you attack" whole-combat
//!   trigger condition + the intervening-if is unexpressible.)
//!
//! Vehicle is an artifact subtype; the keyword grant filters on
//! `with_subtype_sym(vehicle)` controlled by you. (Crewed Vehicles are
//! creatures; the layer-6 grant applies whenever the object is a creature, the
//! faithful reading.)

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fearless Swashbuckler");
    let fish = reg.interner_mut().intern("Fish");
    let pirate = reg.interner_mut().intern("Pirate");
    // Intern "Vehicle" now so the effect fn's lookup is guaranteed to hit.
    let _vehicle = reg.interner_mut().intern("Vehicle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: "Whenever you attack, if a Pirate and a Vehicle attacked this
    // combat, …" has no matching trigger condition (no "you attack"
    // whole-combat trigger) and the intervening-if is unexpressible.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_vehicle_haste,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "Vehicles you control have haste", anchored to this
/// creature.
fn etb_install_vehicle_haste(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let vehicle = reg
        .interner()
        .lookup("Vehicle")
        .expect("Vehicle interned during register()");
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_keyword(
            trig.source,
            ObjectFilter::creature()
                .controlled_by(ControllerConstraint::You)
                .with_subtype_sym(vehicle),
            KeywordAbility::Haste,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
