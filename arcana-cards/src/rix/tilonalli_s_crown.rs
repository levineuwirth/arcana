//! Tilonalli's Crown — `{1}{R}` enchantment — Aura.
//! "Enchant creature. When this Aura enters, it deals 1 damage to
//!  enchanted creature. Enchanted creature gets +3/+0 and has trample."
//!
//! The ETB trigger reaches the host via `source.attached_to` to deal 1
//! damage to it, then installs the +3/+0 `attached_pt` and the trample
//! `attached_keyword` (the exemplar reach-the-host pattern from
//! Runner's Bane).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
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
    let name = reg.interner_mut().intern("Tilonalli's Crown");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
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
    let mut effects = Vec::new();
    if let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) {
        effects.push(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Object(host),
            amount: 1,
        });
    }
    effects.push(Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            3,
            0,
            Duration::WhileSourceOnBattlefield,
        ),
    });
    effects.push(Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_keyword(
            trig.source,
            KeywordAbility::Trample,
            Duration::WhileSourceOnBattlefield,
        ),
    });
    effects
}
