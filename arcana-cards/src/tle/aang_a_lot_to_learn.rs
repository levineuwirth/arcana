//! Aang, A Lot to Learn — `{2}{G/W}` 3/2 Legendary Human Avatar Ally.
//!
//! Aang has vigilance as long as there's a Lesson card in your graveyard.
//! Whenever another creature you control dies, put a +1/+1 counter on Aang.
//!
//! The first line is a conditional static keyword grant (vigilance
//! gated on a graveyard predicate) — a pure static with no trigger
//! word and no activation cost, GAP'd. The second is a death-trigger
//! that grows Aang, wired as a `ZoneChange` (battlefield → graveyard)
//! trigger restricted to creatures you control.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aang, A Lot to Learn");
    let human = reg.interner_mut().intern("Human");
    let avatar = reg.interner_mut().intern("Avatar");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(avatar);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: static "Aang has vigilance as long as there's a Lesson card
    // in your graveyard" — a conditional continuous keyword grant (pure
    // static, no trigger/cost) that the triggered/activated primitives
    // cannot express.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: grow_aang,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Whenever another creature you control dies, put a +1/+1 counter on Aang.
fn grow_aang(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
