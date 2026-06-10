//! Lobe Lobber — `{2}` artifact — Equipment.
//! "Equipped creature has \"{T}: This creature deals 1 damage to target
//! player or planeswalker. Roll a six-sided die. On a 5 or higher,
//! untap it.\" Equip {2}"
//!
//! The Equip half is wired via `with_equip`. The static grants the
//! equipped creature an ACTIVATED ability (with a die roll) — there is
//! no attached-ability grant in the Equipment surface, so the ETB
//! install is an honest no-op with a GAP comment.

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
    let name = reg.interner_mut().intern("Lobe Lobber");
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

/// The printed static grants the equipped creature an activated ability
/// ("{T}: deal 1 damage to target player or planeswalker. Roll a
/// six-sided die. On a 5 or higher, untap it.") — no P/T component.
fn etb_install_static(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: 'equipped creature has "<activated ability>"' — attached_pt
    // covers P/T only (no attached activated-ability grant; the die-roll
    // rider is also unmodeled).
    Vec::new()
}
