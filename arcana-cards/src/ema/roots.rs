//! Roots — `{3}{G}` enchantment — Aura.
//! "Enchant creature without flying. When this Aura enters, tap enchanted
//!  creature. Enchanted creature doesn't untap during its controller's
//!  untap step."
//!
//! Lock Aura. The ETB trigger taps the host (reached via
//! `source.attached_to`) and installs the don't-untap restriction.

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Roots");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        // NOTE: "without flying" restriction approximated to any creature
        // (no keyword-exclusion enchant filter).
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

fn etb_install(state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    let mut effects = vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_dont_untap(
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }];
    if let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) {
        effects.push(Effect::Tap { target: host });
    }
    effects
}
