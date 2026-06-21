//! Mr. Foxglove — `{2}{G}{W}{U}` 3/5 Legendary Fox Rogue with Lifelink.
//!
//! Oracle:
//! * Lifelink
//! * Whenever Mr. Foxglove attacks, draw cards equal to the number of cards in
//!   defending player's hand minus the number of cards in your hand. If you
//!   didn't draw cards this way, you may put a creature card from your hand
//!   onto the battlefield.
//!
//! Lifelink is a base keyword. The attack trigger computes the draw amount at
//! resolution (defending player's hand size minus yours, floored at 0); if
//! that amount is zero ("you didn't draw cards this way") it instead lets you
//! put a creature card from your hand onto the battlefield.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mr. Foxglove");
    let fox = reg.interner_mut().intern("Fox");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fox);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: on_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_attack(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let my_hand = script::hand_size(state, trig.controller) as i32;
    let n = match trig.defending_player() {
        Some(def) => (script::hand_size(state, def) as i32 - my_hand).max(0) as u32,
        None => 0,
    };
    if n > 0 {
        vec![Effect::DrawCards { player: trig.controller, count: n }]
    } else {
        // "If you didn't draw cards this way, you may put a creature card from
        // your hand onto the battlefield."
        vec![Effect::PutFromHandOntoBattlefield {
            player: trig.controller,
            filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            tapped: false,
        }]
    }
}
