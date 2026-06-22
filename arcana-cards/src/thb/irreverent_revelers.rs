//! Irreverent Revelers — `{2}{R}` 2/2 red Satyr.
//! "When this creature enters, choose one —
//!   • Destroy target artifact.
//!   • This creature gains haste until end of turn."
//!
//! The ETB trigger is wired, but its effect is a modal ("choose one")
//! payload.
//! GAP: modal ("choose one") ETB — triggered abilities have no
//!      modal/choose-one machinery in the engine API (modal is a
//!      SpellAbilityDef-only feature), so the choice-bearing effect (and
//!      its mode-1 target) cannot be expressed. Effect returns Vec::new().

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
    let name = reg.interner_mut().intern("Irreverent Revelers");
    let satyr = reg.interner_mut().intern("Satyr");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(satyr);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
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
    // GAP: modal ("choose one — Destroy target artifact / this creature
    // gains haste") ETB — no modal machinery on triggered abilities.
    Vec::new()
}
