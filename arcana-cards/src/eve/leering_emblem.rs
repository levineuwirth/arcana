//! Leering Emblem — `{2}` artifact — Equipment. "Whenever you cast a
//! spell, equipped creature gets +2/+2 until end of turn. Equip {2}"
//!
//! The Equip activation is wired via `with_equip`; the cast trigger is
//! wired on `SpellCast`, but its pump is GAP'd — there is no accessor
//! for the creature this Equipment is currently attached to, so the
//! +2/+2 cannot be aimed at the equipped creature.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Leering Emblem");
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
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pump_equipped_on_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "…equipped creature gets +2/+2 until end of turn."
fn pump_equipped_on_cast(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "equipped creature gets +2/+2 until end of turn" — no
    // accessor exposes the creature this Equipment is attached to
    // (`attached_to` is not readable from the effect fn), so the pump
    // cannot be aimed. Firing Effect::Pump at trig.source would hit the
    // Equipment itself, which is wrong.
    Vec::new()
}
