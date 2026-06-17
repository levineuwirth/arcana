//! A-Baleful Beholder — `{4}{B}{B}` 7/5 Beholder.
//!
//! When Baleful Beholder enters, choose one —
//! • Antimagic Cone — Each opponent sacrifices an enchantment.
//! • Fear Ray — Creatures you control gain menace until end of turn.
//!
//! The ETB trigger is wired (SelfEntersBattlefield), but a modal "choose one"
//! is a spell feature with no triggered-ability form, so the modal choice and
//! both mode payloads are GAP'd.

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
    let name = reg.interner_mut().intern("A-Baleful Beholder");
    let beholder = reg.interner_mut().intern("Beholder");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beholder);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_modal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_modal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal "choose one" has no triggered-ability form — cannot express
    // the per-mode choice between "each opponent sacrifices an enchantment"
    // and "creatures you control gain menace until end of turn".
    Vec::new()
}
