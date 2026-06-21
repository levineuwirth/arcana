//! Vaultbreaker — `{3}{R}` 4/2 Orc Rogue.
//! Whenever this creature attacks, you may discard a card. If you do, draw a
//! card.
//! Dash {2}{R}.
//!
//! Dash is not an emittable KeywordAbility (its alternate-cast-with-haste-
//! and-bounce mode is unmodeled) — GAP'd.
//! The attack trigger is GAP'd: "you may discard a card. If you do, draw a
//! card" is an optional discard gate, and OptionalPayment only models
//! Mana/Life payments (no discard-as-optional-cost), so the may-discard-
//! then-draw cannot be expressed faithfully.

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
    let name = reg.interner_mut().intern("Vaultbreaker");
    let orc = reg.interner_mut().intern("Orc");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: Dash {2}{R} — not an emittable KeywordAbility; alternate cast mode
    // unmodeled.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_loot,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_loot(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may discard a card. If you do, draw a card" — optional
    // discard gate; OptionalPayment only models Mana/Life costs.
    Vec::new()
}
