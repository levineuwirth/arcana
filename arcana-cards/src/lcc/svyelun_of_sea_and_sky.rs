//! Svyelun of Sea and Sky — `{1}{U}{U}` 3/4 Legendary Creature — Merfolk God.
//!
//! Oracle:
//! * Svyelun has indestructible as long as you control at least two other
//!   Merfolk.
//! * Whenever Svyelun attacks, draw a card.
//! * Other Merfolk you control have ward {1}.
//!
//! Decomposition: only the attack trigger (draw a card) is an expressible
//! triggered ability.
//!
//! GAP: "indestructible as long as you control at least two other Merfolk" is
//!      a conditional continuous self-static — not a triggered/activated
//!      ability; omitted.
//! GAP: "Other Merfolk you control have ward {1}" is a static keyword-granting
//!      anthem over other permanents — not expressible here; omitted.

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
    let name = reg.interner_mut().intern("Svyelun of Sea and Sky");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: draw_a_card,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn draw_a_card(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
