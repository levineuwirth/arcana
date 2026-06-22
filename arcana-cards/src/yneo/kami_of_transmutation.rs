//! Kami of Transmutation — `{1}{W}` 2/2 Creature — Spirit.
//! When Kami of Transmutation enters or leaves the battlefield, choose one —
//!   • Each permanent card in your hand perpetually becomes an artifact in
//!     addition to its other types.
//!   • Each permanent card in your hand perpetually becomes an enchantment in
//!     addition to its other types.
//!
//! Decomposition:
//! - "enters or leaves" → two triggers (SelfEntersBattlefield + SelfLeavesBattlefield).
//! - The modal payload ("each permanent card in your hand perpetually becomes
//!   an artifact / enchantment in addition to its other types") is NOT
//!   expressible: there is no Effect that perpetually modifies the types of
//!   CARDS IN HAND, and triggered abilities have no modal-dispatch surface.
//!   Both trigger bodies are GAP'd (return Vec::new()) but the trigger
//!   conditions are still recorded faithfully.

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
    let name = reg.interner_mut().intern("Kami of Transmutation");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: transmute_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfLeavesBattlefield,
                intervening_if: None,
                effect: transmute_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn transmute_hand(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose one — each permanent card in your hand perpetually becomes
    // an artifact / enchantment in addition to its other types." No Effect
    // perpetually retypes cards in hand, and triggered abilities have no modal
    // dispatch. Effect omitted.
    Vec::new()
}
