//! Kotis, Sibsig Champion — `{B}{G}{U}` 3/3 Legendary Zombie Warrior.
//!
//! Once during each of your turns, you may cast a creature spell from your
//! graveyard by exiling three other cards from your graveyard in addition to
//! paying its other costs.
//! Whenever one or more creatures you control enter, if one or more of them
//! entered from a graveyard or was cast from a graveyard, put two +1/+1
//! counters on Kotis.
//!
//! The graveyard-cast permission static has no expressible primitive (GAP).
//! The enter trigger is wired as a creature-you-control ZoneChange FROM a
//! graveyard to the battlefield (this captures the "entered from a graveyard"
//! case; the "was cast from a graveyard" case is a GAP), putting two +1/+1
//! counters on Kotis.

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
    let name = reg.interner_mut().intern("Kotis, Sibsig Champion");
    let zombie = reg.interner_mut().intern("Zombie");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Once during each of your turns, you may cast a creature spell from
    // your graveyard by exiling three other cards…" — graveyard-cast
    // permission static, not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // The "entered from a graveyard" case is captured via from:
            // Graveyard. GAP: the "was cast from a graveyard" alternative is
            // not separately expressible.
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                from: Some(Zone::Graveyard(0)),
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: add_two_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn add_two_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}
