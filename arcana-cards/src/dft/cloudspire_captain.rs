//! Cloudspire Captain — `{2}{W}` 2/3 Human Pilot.
//!
//! "Mounts and Vehicles you control get +1/+1." — WIRED as an ETB-installed
//! `ContinuousEffect::filtered_pump` (filter: creatures you control that are a
//! Mount or a Vehicle), lasting while Cloudspire Captain is on the battlefield
//! (glorious_anthem precedent).
//! "This creature saddles Mounts and crews Vehicles as though its power were 2
//! greater." — a saddle/crew-cost modification static; no primitive on this
//! shape (GAP).

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Cloudspire Captain");
    let human = reg.interner_mut().intern("Human");
    let pilot = reg.interner_mut().intern("Pilot");
    // Intern the anthem-filter subtypes.
    let _mount = reg.interner_mut().intern("Mount");
    let _vehicle = reg.interner_mut().intern("Vehicle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(pilot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "saddles Mounts and crews Vehicles as though its power were 2
    //      greater" — saddle/crew-cost modification static, unexpressible.
    reg.register(
        CardDefinition::new(name, chars)
            // "Mounts and Vehicles you control get +1/+1." installed on ETB.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_mount_vehicle_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Install "Mounts and Vehicles you control get +1/+1" anchored to Cloudspire
/// Captain, lasting while it remains on the battlefield.
fn install_mount_vehicle_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mount = reg.interner().lookup("Mount").unwrap_or_default();
    let vehicle = reg.interner().lookup("Vehicle").unwrap_or_default();
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![mount, vehicle]);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            filter,
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
