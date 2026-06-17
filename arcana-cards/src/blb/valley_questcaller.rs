//! Valley Questcaller — `{1}{W}` 2/3 Rabbit Warrior.
//!
//! Oracle:
//! * Whenever one or more other Rabbits, Bats, Birds, and/or Mice you control
//!   enter, scry 1.
//! * Other Rabbits, Bats, Birds, and Mice you control get +1/+1. (static
//!   anthem — GAP: not expressible.)

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Valley Questcaller");
    let rabbit = reg.interner_mut().intern("Rabbit");
    let warrior = reg.interner_mut().intern("Warrior");
    let bat = reg.interner_mut().intern("Bat");
    let bird = reg.interner_mut().intern("Bird");
    let mouse = reg.interner_mut().intern("Mouse");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rabbit);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // Trigger filter: a creature you control of one of the named subtypes.
    // ("one or more ... enter" is approximated by firing per matching
    // creature; the "other" exclusion of Lita herself is not on the filter.)
    let kindred_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![rabbit, bat, bird, mouse]);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: kindred_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: scry_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
    // GAP: "Other Rabbits, Bats, Birds, and Mice you control get +1/+1." —
    // static anthem is not expressible with the demonstrated API.
}

fn scry_one(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Scry { player: trig.controller, count: 1 }]
}
