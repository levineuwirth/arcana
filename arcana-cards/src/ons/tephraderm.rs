//! Tephraderm — `{4}{R}` 4/5 red Beast.
//!
//! Oracle:
//! * Whenever a creature deals damage to this creature, this creature deals
//!   that much damage to that creature.
//! * Whenever a spell deals damage to this creature, this creature deals that
//!   much damage to that spell's controller.
//!
//! Both triggers fire on `SelfIsDealtDamage`. The damage AMOUNT is reachable
//! via `trig.damage_amount()`, but the SOURCE of the damage ("that creature" /
//! "that spell's controller") is not exposed by any `PendingTrigger`
//! accessor. Without the dealing object's id there is no way to reflect the
//! damage back, so both effect bodies are GAP'd while the triggers remain
//! registered.

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
    let name = reg.interner_mut().intern("Tephraderm");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfIsDealtDamage { combat_only: false },
                intervening_if: None,
                effect: reflect_to_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfIsDealtDamage { combat_only: false },
                intervening_if: None,
                effect: reflect_to_spell_controller,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn reflect_to_creature(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "this creature deals that much damage to that creature" — the
    // dealing creature's id is not exposed (no damage-source accessor on
    // PendingTrigger). damage_amount() gives the quantity but not the target.
    Vec::new()
}

fn reflect_to_spell_controller(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "this creature deals that much damage to that spell's controller" —
    // no accessor exposes the damaging spell or its controller.
    Vec::new()
}
