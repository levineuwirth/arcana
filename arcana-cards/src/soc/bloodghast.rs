//! Bloodghast — `{B}{B}` 2/1 Vampire Spirit.
//! "This creature can't block.
//!  This creature has haste as long as an opponent has 10 or less life.
//!  Landfall — Whenever a land you control enters, you may return this
//!  card from your graveyard to the battlefield."
//!
//! The Landfall recursion trigger is wired (a land you control entering
//! returns this card from the graveyard to the battlefield). The two
//! statics — "can't block" and the life-gated conditional haste — are
//! continuous statics, not triggered/activated abilities, and are GAP'd.

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

// GAP: static — "This creature can't block" is a continuous static restriction.
// GAP: static — "has haste as long as an opponent has 10 or less life" is a
// life-gated continuous static.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodghast");
    let vampire = reg.interner_mut().intern("Vampire");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: landfall_reanimate,
                // This ability triggers from the graveyard.
                trigger_zones: vec![Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn landfall_reanimate(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "you may return this card from your graveyard to the battlefield"
    vec![Effect::ReturnFromGraveyardToBattlefield { target: trig.source }]
}
