//! Earth-Cult Elemental — `{4}{R}{R}` 6/6 Elemental.
//!
//! Oracle:
//!  * Siege Monster — When this creature enters, roll a d20.
//!      1—9   | Each player sacrifices a permanent of their choice.
//!      10—19 | Each opponent sacrifices a permanent of their choice.
//!      20    | Each opponent sacrifices two permanents of their choice.
//!
//! "Siege Monster" is an ability word (flavor). The ETB rolls a d20 and
//! branches on the result; there is no die-roll Effect variant, so the whole
//! roll-and-dispatch is GAP'd.

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
    let name = reg.interner_mut().intern("Earth-Cult Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_roll_d20,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_roll_d20(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "roll a d20" with a result-keyed sacrifice table has no die-roll
    // Effect variant; the random roll and its branching cannot be expressed.
    Vec::new()
}
