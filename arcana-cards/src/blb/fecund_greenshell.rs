//! Fecund Greenshell — {3}{G}{G} 4/6 Creature — Elemental Turtle.
//! Reach.
//! As long as you control ten or more lands, creatures you control get +2/+2
//! (GAP — static continuous anthem is not a triggered/activated ability).
//! Whenever this or another creature you control with toughness greater than
//! its power enters, look at the top card of your library; if a land, you may
//! put it onto the battlefield tapped, otherwise put it into your hand.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
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
    let name = reg.interner_mut().intern("Fecund Greenshell");
    let elemental = reg.interner_mut().intern("Elemental");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(turtle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };
    // GAP: "as long as you control ten or more lands, creatures you control
    // get +2/+2" is a static continuous anthem, not a triggered/activated
    // ability — omitted.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP (filter): "with toughness greater than its power" is not an
            // expressible ObjectFilter predicate — over-fires on any creature
            // you control entering.
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: peek_top_land_or_hand,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn peek_top_land_or_hand(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at the top card; if a land, you may put it onto the
    // battlefield tapped, otherwise into your hand" has no single matching
    // primitive (the conditional play-or-hand routing is unexpressible).
    Vec::new()
}
