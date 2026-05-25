//! Town Greeter — `{1}{G}` 1/1 Human Citizen.
//! Keywords: Mill
//! "When this creature enters, mill four cards. You may put a land
//! card from among them into your hand. If you put a Town card into
//! your hand this way, you gain 2 life."
//!
//! GAP: "you may put a land card from among the milled cards into
//! your hand" — selecting a specific card from the just-milled cards
//! is not expressible with catalog variants after a Mill effect.
//! Also "if you put a Town card into your hand this way" is a
//! conditional based on the player's choice.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Town Greeter");
    let human = reg.interner_mut().intern("Human");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(citizen);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_etb(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // Mill four is expressible; the rest is a GAP.
    // GAP: "put a land card from among the milled cards into your hand"
    // — cannot select specific cards from just-milled group.
    // GAP: "if you put a Town card into your hand" — conditional on
    // player choice not expressible.
    vec![Effect::Mill { player: trig.controller, count: 4 }]
}
