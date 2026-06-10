//! Sorcerer's Wand — `{1}` artifact — Equipment (Dominaria).
//! "Equipped creature has \"{T}: This creature deals 1 damage to
//! target player or planeswalker. If this creature is a Wizard, it
//! deals 2 damage instead.\" Equip {3}."
//!
//! The Equip half is real; granting the equipped creature an ACTIVATED
//! ability is not expressible (the attached static surface covers P/T
//! only) — the ETB install is GAP'd.

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
    let name = reg.interner_mut().intern("Sorcerer's Wand");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{3}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Equipped creature has '{T}: This creature deals 1 damage to
    // target player or planeswalker. If this creature is a Wizard, it
    // deals 2 damage instead.'" — granting the attached creature an
    // ACTIVATED ability is not expressible (attached_pt covers P/T only;
    // no attached activated-ability grant exists).
    Vec::new()
}
