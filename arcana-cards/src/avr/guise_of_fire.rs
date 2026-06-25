//! Guise of Fire — `{R}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +1/-1 and attacks each
//! combat if able."
//!
//! The +1/-1 is an ETB-installed `attached_pt`. "Attacks each combat if
//! able" is wired as an ETB-installed `must_attack` on the enchanted
//! creature (resolved from this Aura's `attached_to`; CR 508.1a).

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
    let name = reg.interner_mut().intern("Guise of Fire");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
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
                effect: etb_install_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_pump(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            1,
            -1,
            Duration::WhileSourceOnBattlefield,
        ),
    }];
    // "Enchanted creature ... attacks each combat if able." (CR 508.1a) —
    // installed on the host resolved from this Aura's attachment.
    if let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) {
        effects.push(Effect::InstallContinuousEffect {
            effect: ContinuousEffect::must_attack(
                trig.source,
                host,
                Duration::WhileSourceOnBattlefield,
            ),
        });
    }
    effects
}
