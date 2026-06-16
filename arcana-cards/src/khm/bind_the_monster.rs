//! Bind the Monster — `{U}` enchantment — Aura.
//! "Enchant creature. When this Aura enters, tap enchanted creature. It
//!  deals damage to you equal to its power. Enchanted creature doesn't
//!  untap during its controller's untap step."
//!
//! The ETB taps the host (reached via `source.attached_to`), deals damage
//! to the Aura's controller equal to the host's power, and installs the
//! `attached_dont_untap` lock.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bind the Monster");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
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
    let Some(host) = state.objects.get(trig.source).and_then(|o| o.attached_to) else {
        return Vec::new();
    };
    let power = script::power_of(state, host).max(0) as u32;
    vec![
        Effect::Tap { target: host },
        Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(trig.controller),
            amount: power,
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_dont_untap(
                trig.source,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
