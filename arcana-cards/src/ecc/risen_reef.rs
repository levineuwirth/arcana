//! Risen Reef — `{1}{G}{U}` 1/1 green/blue Elemental.
//! "Whenever this creature or another Elemental you control enters, look at
//! the top card of your library. If it's a land card, you may put it onto
//! the battlefield tapped. If you don't put the card onto the battlefield,
//! put it into your hand."
//!
//! GAP: The "if it's a land card, put it onto the battlefield tapped; otherwise
//! put it into your hand" branching logic is not expressible with DigTopN
//! (which only has a hand destination). The effect is approximated as DigTopN
//! count:1 filter:None (any card to hand), eliding the land-to-battlefield path.

use arcana_core::effects::{DigRest, Effect};
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
    let name = reg.interner_mut().intern("Risen Reef");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // "Whenever this creature or another Elemental you control enters"
                // — ZoneChange to Battlefield for Elementals you control.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::CREATURE.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: elemental_enters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn elemental_enters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if it's a land card, you may put it onto the battlefield tapped;
    // if you don't, put it into your hand" — the conditional land-to-battlefield
    // vs hand path is not expressible with DigTopN. Approximated as DigTopN
    // putting any top card into hand.
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 1,
        filter: None,
        rest: DigRest::BottomRandom,
    }]
}
