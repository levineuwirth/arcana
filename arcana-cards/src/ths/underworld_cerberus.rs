//! Underworld Cerberus — `{3}{B}{R}` 6/6 Dog.
//! Can't be blocked except by three or more creatures. (static)
//! Cards in graveyards can't be the targets of spells or abilities. (static)
//! When this creature dies, exile it and each player returns all creature cards
//! from their graveyard to their hand.

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
    let name = reg.interner_mut().intern("Underworld Cerberus");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    // GAP: static "can't be blocked except by three or more creatures" — no block-restriction-by-count primitive.
    // GAP: static "cards in graveyards can't be the targets of spells or abilities" — no targeting-restriction primitive.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: on_dies,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_dies(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // Exile this creature, then each player returns all creature cards from their
    // graveyard to their hand.
    // GAP: "each player returns all creature cards from their graveyard to their hand" — there is
    // no scripting helper to enumerate creature CARDS in a graveyard, so the mass-return is omitted.
    let _ = state;
    vec![Effect::ExilePermanent { target: trig.source }]
}
