//! Locked in the Cemetery — `{1}{U}` enchantment — Aura.
//! "Enchant creature.
//!  When this Aura enters, if there are five or more cards in your
//!  graveyard, tap enchanted creature.
//!  Enchanted creature doesn't untap during its controller's untap
//!  step."
//!
//! The static is attached_dont_untap, installed unconditionally on ETB.
//! The conditional ETB tap is gated by a graveyard-size check in the
//! body (the "if five or more" clause applies only to the tap, not to
//! the dont_untap install, so the gate is checked in-effect).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::conditions;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Locked in the Cemetery");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
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
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_dont_untap(
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }];
    // Conditional ETB tap: only if five or more cards in your graveyard.
    if conditions::graveyard_at_least(state, trig.controller, 5) {
        if let Some(host) =
            state.object_or_lki(trig.source).and_then(|o| o.attached_to)
        {
            effects.push(Effect::Tap { target: host });
        }
    }
    effects
}
