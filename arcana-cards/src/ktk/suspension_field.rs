//! Suspension Field — `{1}{W}` enchantment (Khans of Tarkir, 2014).
//! "When this enchantment enters, you may exile target creature with
//! toughness 3 or greater until this enchantment leaves the battlefield."
//!
//! ETB trigger exiling the targeted creature (the "you may" is resolved as
//! a yes). GAPs: the toughness-3-or-greater target restriction (only
//! `with_max_toughness` exists, no minimum) and the O-Ring return when this
//! enchantment leaves (DelayedAction cannot key one object's action to a
//! DIFFERENT object's zone change).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Suspension Field");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: suspend_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // GAP: "with toughness 3 or greater" — ObjectFilter has
                // with_max_toughness only (no minimum-toughness predicate);
                // plain creature target declared.
                target_requirements: vec![TargetRequirement::target_creature()],
            },
        ),
    )
}

/// "…you may exile target creature … until this enchantment leaves the
/// battlefield."
fn suspend_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "until this enchantment leaves the battlefield" — the return
    // half needs a delayed action on the EXILED card keyed to THIS
    // enchantment's departure; DelayedAction only supports
    // NextEndStep/ThisDies on a single id, so the exile is permanent here.
    vec![Effect::ExilePermanent { target: *id }]
}
