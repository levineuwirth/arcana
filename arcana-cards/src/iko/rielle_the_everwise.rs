//! Rielle, the Everwise — `{1}{U}{R}` 0/3 Legendary Human Wizard.
//!
//! Oracle:
//! * Rielle gets +1/+0 for each instant and sorcery card in your
//!   graveyard. (static characteristic-defining P/T boost — GAP)
//! * Whenever you discard one or more cards for the first time each turn,
//!   draw that many cards.
//!
//! The dynamic self-buff is a static (no trigger/cost) and there is no
//! dynamic self-P/T primitive — GAP'd. The first-discard draw is wired as
//! a once-per-turn CardDiscarded trigger drawing the number of cards
//! discarded this turn (which, on the first discard of the turn, equals
//! that batch — a documented fidelity approximation).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rielle, the Everwise");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Rielle gets +1/+0 for each instant and sorcery card in your
    //       graveyard." (static dynamic self-P/T)
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CardDiscarded {
                player: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: draw_that_many,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::OncePerTurn,
            target_requirements: Vec::new(),
        }),
    )
}

fn draw_that_many(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let n = script::cards_discarded_this_turn(state, trig.controller);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::DrawCards {
        player: trig.controller,
        count: n,
    }]
}
