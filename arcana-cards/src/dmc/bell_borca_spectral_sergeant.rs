//! Bell Borca, Spectral Sergeant — `{2}{R}{W}` */5 Legendary Spirit
//! Soldier.
//! "Note the mana value of each card as it's put into exile."
//! "Bell Borca's power is equal to the greatest number noted for it
//! this turn."
//! "At the beginning of your upkeep, exile the top card of your
//! library. You may play that card this turn."
//!
//! Power is `*` (a characteristic-defining ability whose value is the
//! greatest mana value noted from exiles this turn). The noting
//! bookkeeping and the CDA tie-in have no expressible primitive — GAP'd
//! (power emitted as `PtValue::Star`, base value 0). The upkeep ability
//! is a faithful impulse-exile of one card playable this turn.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bell Borca, Spectral Sergeant");
    let spirit = reg.interner_mut().intern("Spirit");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(soldier);
    // GAP (CDA power): "Bell Borca's power equals the greatest mana value
    // noted for it this turn." No mana-value-noting bookkeeping nor a CDA
    // hook — power is left as `*` (Star) with no value source.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_impulse,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_impulse(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ImpulseExile { player: trig.controller, count: 1 }]
}
