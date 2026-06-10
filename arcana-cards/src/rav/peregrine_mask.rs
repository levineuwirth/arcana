//! Peregrine Mask — `{1}` artifact — Equipment.
//! "Equipped creature has defender, flying, and first strike. Equip `{2}`."
//! The Equip half is wired via `with_equip`; the keyword-only static is a
//! documented gap (no attached keyword grant yet).

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
    let name = reg.interner_mut().intern("Peregrine Mask");
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

/// The printed static is keyword-only ("equipped creature has defender,
/// flying, and first strike") — there is no attached keyword grant, so
/// nothing is installed.
fn etb_install_static(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: 'equipped creature has defender, flying, and first strike' —
    // attached_pt covers P/T only (no attached keyword grant yet)
    Vec::new()
}
