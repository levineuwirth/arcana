//! Bow of the Hunter — Hero Artifact — Equipment. "Equipped creature
//! has \"{T}: This creature deals 2 damage to any target.\"
//! Equip {2}"
//!
//! The granted activated ability is not expressible — the Equipment
//! static surface covers only `attached_pt`; the ETB install fn
//! returns nothing with a GAP note. The Hero card type is not a
//! `TypeLine` const; emitted as a plain artifact.

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
    let name = reg.interner_mut().intern("Bow of the Hunter");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    // GAP: "Hero" card type (Theros Hero's Path) — no TypeLine const exists;
    // emitted as a plain artifact. The spec lists no mana cost.
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{2}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_static,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_static(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "equipped creature has '{T}: This creature deals 2 damage to any
    // target.'" — granting an activated ability to the attached creature is
    // not expressible (attached_pt covers P/T only).
    Vec::new()
}
