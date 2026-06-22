//! Stone Haven Outfitter — `{1}{W}` 2/2 Creature — Kor Artificer Ally.
//! Equipped creatures you control get +1/+1.
//! Whenever an equipped creature you control dies, draw a card.

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stone Haven Outfitter");
    let kor = reg.interner_mut().intern("Kor");
    let artificer = reg.interner_mut().intern("Artificer");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kor);
    subtypes.0.insert(artificer);
    subtypes.0.insert(ally);

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

    // GAP (static): "Equipped creatures you control get +1/+1" — a static
    // continuous anthem filtered on the "equipped" status; no expressible
    // representation here.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // "an equipped creature you control dies" — the "equipped" status is
            // not an available ObjectFilter predicate (see GAP); modeled as
            // "a creature you control dies", which over-fires.
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: draw_a_card,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn draw_a_card(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the "equipped" qualifier on the dying creature is not expressible in
    // the ObjectFilter — this draws for any creature you control that dies.
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
