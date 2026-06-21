//! Pack Leader — `{1}{W}` 2/2 Dog.
//! "Other Dogs you control get +1/+1." (static anthem — GAP)
//! "Whenever this creature attacks, prevent all combat damage that would be
//!  dealt this turn to Dogs you control."
//!
//! GAP: the +1/+1 Dog anthem is a pure static continuous ability with no
//!      trigger or cost — not expressible as a triggered/activated ability.
//! The attack trigger is wired with `PreventDamageFrom`: source = creatures
//! (combat damage is dealt by creatures), target = Dogs you control.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pack Leader");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "Other Dogs you control get +1/+1." — pure static anthem.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: prevent_damage_to_dogs,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn prevent_damage_to_dogs(
    _state: &GameState,
    _trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dogs = script::subtype_filter(reg, "Dog").controlled_by(ControllerConstraint::You);
    vec![Effect::PreventDamageFrom {
        source_filter: ObjectFilter::creature(),
        target_filter: TargetFilter::Permanent(dogs),
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
