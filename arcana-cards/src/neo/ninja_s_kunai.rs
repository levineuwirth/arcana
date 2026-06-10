//! Ninja's Kunai — `{1}` artifact — Equipment (Kamigawa: Neon
//! Dynasty). "Equipped creature has \"{1}, {T}, Sacrifice Ninja's
//! Kunai: Ninja's Kunai deals 3 damage to any target.\" Equip {1}."
//!
//! Equip {1} via `with_equip`.
//!
//! GAP: the entire static is a granted activated ability on the
//! equipped creature — there is no attached-ability grant
//! (attached_pt covers P/T only), so the ETB install is a stub.

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
    let name = reg.interner_mut().intern("Ninja's Kunai");
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
            .with_equip(ManaCost::parse("{1}").expect("valid cost"))
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
    // GAP: 'equipped creature has "{1}, {T}, Sacrifice Ninja's Kunai:
    // Ninja's Kunai deals 3 damage to any target."' — granted
    // activated abilities on the attached creature are not
    // expressible (attached_pt covers P/T only).
    Vec::new()
}
