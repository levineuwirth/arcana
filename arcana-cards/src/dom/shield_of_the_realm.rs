//! Shield of the Realm — {2} artifact — Equipment. "If a source would
//! deal damage to equipped creature, prevent 2 of that damage.
//! Equip {1}"
//!
//! The static is a continuous damage-prevention replacement on the
//! dynamic equipped creature — not expressible (attached_pt covers P/T
//! only; `Effect::PreventDamage` is a fixed-id, bounded-duration
//! one-shot). The static is a documented gap; the Equip activation
//! itself is fully wired.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shield of the Realm");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{1}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_statics,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_statics(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If a source would deal damage to equipped creature, prevent
    // 2 of that damage" — a continuous prevention shield on the dynamic
    // attached creature is not expressible (attached_pt covers P/T
    // only; PreventDamage is a fixed-id one-shot).
    Vec::new()
}
