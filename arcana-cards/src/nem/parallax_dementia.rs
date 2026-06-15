//! Parallax Dementia — `{1}{B}` enchantment — Aura.
//! "Enchant creature. Fading 1. Enchanted creature gets +3/+2. When this Aura
//!  leaves the battlefield, destroy enchanted creature. That creature can't
//!  be regenerated."
//!
//! Buff Aura with Fading. Fading 1 is a card keyword. The +3/+2 is an
//! ETB-installed `attached_pt`. The "when this Aura leaves the battlefield,
//! destroy enchanted creature" payoff fires off the AURA leaving — a
//! self-leaves trigger that has no expressible condition — and is GAPped.

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
    let name = reg.interner_mut().intern("Parallax Dementia");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        keywords: vec![KeywordAbility::Fading(1)],
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
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "when this Aura leaves the battlefield, destroy enchanted creature
    // (can't be regenerated)" — self-leaves payoff not expressible.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            3,
            2,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
