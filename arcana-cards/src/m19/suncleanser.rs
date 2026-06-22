//! Suncleanser — `{1}{W}` 1/4 Human Cleric.
//! "When this creature enters, choose one —
//!  • Remove all counters from target creature. It can't have counters put
//!    on it for as long as this creature remains on the battlefield.
//!  • Target opponent loses all counters. That player can't get counters
//!    for as long as this creature remains on the battlefield."
//!
//! GAP: the ETB is a MODAL triggered ability ("choose one"). The documented
//! modal machinery (`ModalSpec` / `dispatch_modal_effect`) is only available
//! on `SpellAbilityDef`, not on `TriggeredAbilityDef`, so the mode choice
//! cannot be expressed on this trigger.
//! GAP (mode 1): "remove ALL counters from target creature" has no primitive
//! (`Effect::RemoveCounters` removes a specific kind/count), and the "can't
//! have counters put on it" lock is a static replacement effect — neither
//! expressible.
//! GAP (mode 2): "target opponent loses all counters" + "can't get counters"
//! is a player-counter / static-replacement effect with no documented
//! primitive.
//! The ETB trigger is retained but resolves to a no-op.

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
    let name = reg.interner_mut().intern("Suncleanser");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_modal_counter_strip,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_modal_counter_strip(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal-on-trigger not expressible; both modes use remove-all-counters
    // / counter-lock statics with no documented primitive.
    Vec::new()
}
