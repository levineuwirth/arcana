//! Epochrasite — `{2}` 1/1 Artifact Creature — Construct (colorless).
//!
//! "This creature enters with three +1/+1 counters on it if you didn't cast
//!  it from your hand.
//!  When this creature dies, exile it with three time counters on it and it
//!  gains suspend."
//!
//! The conditional enters-with-counters static keys on the cast-from-hand
//! history of THIS object — no expressible enters-with replacement gated on
//! that condition. The dies trigger's payload ("exile with three time
//! counters and gains suspend") relies on the suspend mechanic, which is not
//! modeled. The dies trigger shape is recorded with a GAP'd payload.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Epochrasite");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP (enters-with): "enters with three +1/+1 counters if you didn't cast
    //      it from your hand" — no enters-with replacement gated on the
    //      cast-from-hand condition.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_exile_with_suspend,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dies_exile_with_suspend(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile it with three time counters on it and it gains suspend" —
    // suspend is not a modeled mechanic and there is no exile-with-counters +
    // grant-suspend primitive.
    Vec::new()
}
