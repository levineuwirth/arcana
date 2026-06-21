//! Vizkopa Confessor — `{3}{W}{B}` 1/3 Human Cleric with Extort.
//!
//! Extort (Whenever you cast a spell, you may pay {W/B}. If you do, each
//! opponent loses 1 life and you gain that much life.)
//! When this creature enters, pay any amount of life. Target opponent
//! reveals that many cards from their hand. You choose one of them and
//! exile it.
//!
//! Extort is not an available `KeywordAbility` variant for this card class
//! (only the listed evergreen / parametrized keywords are usable), so it is
//! GAP'd. The ETB "pay any amount of life, then a variable-count
//! reveal-and-exile" cannot be expressed with the demonstrated API (no
//! variable life-payment-into-amount primitive), so its effect is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::mana::ManaCost;
use arcana_core::targets::{
    ControllerConstraint, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vizkopa Confessor");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    // GAP: keyword — Extort is not an available KeywordAbility variant.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_reveal_and_exile,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Player,
                count: TargetCount::Exactly(1),
                controller: Some(ControllerConstraint::Opponent),
            }],
        }),
    )
}

fn etb_reveal_and_exile(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "pay any amount of life. Target opponent reveals that many cards
    // from their hand. You choose one and exile it." — no primitive ties a
    // variable life payment to a reveal-count, and there is no
    // reveal-hand-and-exile-chosen effect. Targeting kept; effect declined.
    Vec::new()
}
