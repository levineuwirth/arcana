//! Scrapwork Mutt — `{2}` 2/1 Artifact Creature — Dog.
//!
//! Oracle:
//! * "When this creature enters, you may discard a card. If you do,
//!   draw a card." — ETB loot, wired as an OptionalPayment (Discard 1 →
//!   DrawCards 1; decline does nothing).
//! * Unearth {1}{R} — GAP: Unearth is not a `KeywordAbility` variant
//!   (the graveyard alt-cast cast mechanic is not modeled); emitted as
//!   no keyword.

use arcana_core::actions::OptionalPaymentKind;
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
    let name = reg.interner_mut().intern("Scrapwork Mutt");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Unearth {1}{R} — not a KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_loot,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_loot(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "You may discard a card. If you do, draw a card."
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Discard(1),
        then: Box::new(Effect::DrawCards { player: trig.controller, count: 1 }),
        else_effect: None,
    }]
}
