//! Uncontrollable Anger — `{2}{R}{R}` enchantment — Aura.
//! "Flash. Enchant creature. Enchanted creature gets +2/+2 and attacks each
//!  combat if able."
//!
//! Flash is a keyword. The +2/+2 is an ETB-installed `attached_pt`. "Attacks
//! each combat if able" is a must-attack requirement (CR 508.1a) installed on
//! the enchanted creature (the Aura's host) — shed with the Aura.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Uncontrollable Anger");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        keywords: vec![KeywordAbility::Flash],
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

fn etb_install(state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    let mut effects = vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            2,
            2,
            Duration::WhileSourceOnBattlefield,
        ),
    }];
    // "attacks each combat if able" — a must-attack on the enchanted creature.
    // The Aura is already attached when this ETB trigger resolves (CR 303.4f),
    // so its host is the must-attack target; shed with the Aura.
    if let Some(host) = state.objects.get(trig.source).and_then(|o| o.attached_to) {
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
