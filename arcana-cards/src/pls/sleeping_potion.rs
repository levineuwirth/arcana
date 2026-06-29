//! Sleeping Potion — `{1}{U}` enchantment — Aura (MIR).
//! "Enchant creature. When Sleeping Potion enters, tap enchanted creature.
//!  Enchanted creature doesn't untap during its controller's untap step.
//!  When enchanted creature becomes the target of a spell or ability, sacrifice
//!  Sleeping Potion."
//!
//! GAP (partial): The "sacrifice Sleeping Potion when enchanted creature becomes
//! the target" rider uses AttachedCreatureDoes + SelfBecomesTarget, but the
//! response is to sacrifice a SPECIFIC OBJECT (the Aura itself), which requires
//! a targeted sacrifice; the current Sacrifice effect picks from a filter, not a
//! named object.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sleeping Potion");
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
                effect: etb_tap_and_dont_untap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfBecomesTarget {
                        caster: ControllerConstraint::Any,
                    }),
                },
                intervening_if: None,
                effect: host_becomes_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tap_and_dont_untap(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    // Tap the enchanted creature.
    if let Some(obj) = state.objects.get(trig.source) {
        if let Some(host_id) = obj.attached_to {
            effects.push(Effect::Tap { target: host_id });
        }
    }
    effects.push(Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_dont_untap(
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    });
    effects
}

fn host_becomes_target(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: sacrifice THIS specific aura — Sacrifice effect filters by ObjectFilter,
    // not by a specific ObjectId; no "sacrifice self" Effect variant exists.
    Vec::new()
}
