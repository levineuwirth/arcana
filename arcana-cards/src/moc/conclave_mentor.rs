//! Conclave Mentor — `{G}{W}` 2/2 Centaur Cleric.
//! "If one or more +1/+1 counters would be put on a creature you control,
//!  that many plus one +1/+1 counters are put on that creature instead."
//! "When this creature dies, you gain life equal to its power."
//!
//! The counter-replacement static has no expressible primitive (GAP). The
//! dies trigger is expressed; "its power" reads the dying object's power.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Conclave Mentor");
    let centaur = reg.interner_mut().intern("Centaur");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);
    subtypes.0.insert(cleric);

    // GAP: "If one or more +1/+1 counters would be put on a creature you
    // control, that many plus one are put instead." — counter-replacement
    // static is not expressible with the available primitives.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: gain_life_equal_power,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn gain_life_equal_power(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let id = trig.dying_object().unwrap_or(trig.source);
    let n = script::power_of(state, id).max(0) as u32;
    vec![Effect::GainLife {
        player: trig.controller,
        amount: n,
    }]
}
