//! Lazotep Sliver — `{3}{B}` 4/4 Zombie Sliver.
//!
//! "Sliver creatures you control have afflict 2." (static keyword grant —
//!  Afflict is not a supported KeywordAbility variant nor a continuous-grant
//!  primitive; GAP'd.)
//! "Whenever a nontoken Sliver you control dies, amass Slivers 2."
//!  (expressed below via Effect::Amass.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lazotep Sliver");
    let zombie = reg.interner_mut().intern("Zombie");
    let sliver = reg.interner_mut().intern("Sliver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(sliver);

    // Pre-intern subtypes the Amass resolver needs.
    let _army = reg.interner_mut().intern("Army");

    let sliver_filter: ObjectFilter = script::subtype_filter(reg, "Sliver")
        .controlled_by(ControllerConstraint::You)
        .nontoken();

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static — "Sliver creatures you control have afflict 2."

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: sliver_filter,
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: amass_slivers,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn amass_slivers(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let army_subtype = reg.interner().lookup("Army").unwrap_or_default();
    let race_subtype = reg.interner().lookup("Sliver").unwrap_or_default();
    vec![Effect::Amass {
        controller: trig.controller,
        count: 2,
        army_subtype,
        race_subtype,
    }]
}
