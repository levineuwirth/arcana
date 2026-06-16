//! Badgermole — `{4}{G}` 4/4 Badger Mole.
//! "When this creature enters, earthbend 2."
//! "Creatures you control with +1/+1 counters on them have trample."
//!
//! Earthbend has no expressible effect primitive, so the ETB body is
//! GAP'd. The second line is a pure static continuous ability (no
//! trigger/cost) that cannot be expressed as a triggered/activated
//! ability, so it is GAP'd entirely.

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
    let name = reg.interner_mut().intern("Badgermole");
    let badger = reg.interner_mut().intern("Badger");
    let mole = reg.interner_mut().intern("Mole");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(badger);
    subtypes.0.insert(mole);

    // GAP: static "Creatures you control with +1/+1 counters on them have
    // trample" — a pure continuous static, not a triggered/activated ability.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: earthbend,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn earthbend(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "earthbend 2" — animate target land you control into a 0/0 haste
    // creature with two +1/+1 counters plus a recursion trigger; no
    // expressible primitive.
    Vec::new()
}
