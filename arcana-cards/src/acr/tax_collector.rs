//! Tax Collector — `{1}{W}` 2/2 white Human Advisor.
//!
//! Oracle:
//! * When this creature enters, choose one —
//!     • Tax — Until your next turn, spells your opponents cast cost {1} more.
//!     • Arrest — Detain target creature an opponent controls.
//!
//! "Detain" (Scryfall keyword) is an effect keyword, not a base keyword.
//! The ETB is a MODAL trigger; modal selection is only modeled on spell
//! abilities (not triggered abilities) in this shape. The Tax mode is a
//! cost-increase static with no Effect variant, and Detain has no dedicated
//! Effect variant. The whole modal ETB is GAP'd below.

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
    let name = reg.interner_mut().intern("Tax Collector");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_modal_gap,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_modal_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal ETB "choose one — Tax / Arrest". Modal selection is only
    // modeled on spell abilities; neither the cost-increase static (Tax) nor
    // Detain (Arrest) has an expressible Effect variant in this shape.
    Vec::new()
}
