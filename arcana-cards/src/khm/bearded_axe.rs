//! Bearded Axe — `{2}{R}` artifact — Equipment (KHM).
//! "Equipped creature gets +1/+1 for each Dwarf, Equipment, and Vehicle you
//!  control. Equip {2}"

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
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bearded Axe");
    let equipment = reg.interner_mut().intern("Equipment");
    let _dwarf = reg.interner_mut().intern("Dwarf");
    let _vehicle = reg.interner_mut().intern("Vehicle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{2}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_dynamic_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_dynamic_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dwarf = match reg.interner().lookup("Dwarf") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let equipment = match reg.interner().lookup("Equipment") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let vehicle = match reg.interner().lookup("Vehicle") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let filter = ObjectFilter::permanent()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![dwarf, equipment, vehicle]);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt_per_match(
            trig.source,
            filter,
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
