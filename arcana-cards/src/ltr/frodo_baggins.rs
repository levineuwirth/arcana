//! Frodo Baggins — `{G}{W}` 1/3 Legendary Halfling Scout.
//! Whenever Frodo Baggins or another legendary creature you control enters, the
//! Ring tempts you. (The trigger condition is wired; "the Ring tempts you" has no
//! expressible Effect — GAP'd, resolver returns no effects.)
//! The static "must be blocked while your Ring-bearer" is GAP'd (Ring-bearer not modeled).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Frodo Baggins");
    let halfling = reg.interner_mut().intern("Halfling");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: ring_tempts,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
    // GAP: static "As long as Frodo is your Ring-bearer, it must be blocked if able"
    // (Ring-bearer designation not modeled).
}

fn ring_tempts(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "the Ring tempts you" — Ring/temptation mechanic not modeled.
    Vec::new()
}
