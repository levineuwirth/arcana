//! Aquitect's Defenses — `{1}{U}` enchantment — Aura.
//! "Flash. Enchant creature you control. When this Aura enters, enchanted
//!  creature gains hexproof until end of turn. Enchanted creature gets +1/+2."
//!
//! Flash buff Aura. The durable +1/+2 is the standard `attached_pt` install.
//!
//! GAP: ETB one-time "gains hexproof until end of turn" — a single-turn grant
//! on the host is not expressible via the shown attached-grant API (which
//! grants WhileSourceOnBattlefield durations only).

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
    let name = reg.interner_mut().intern("Aquitect's Defenses");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // NOTE: controller wording approximated by caster's choice.
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

fn etb_install(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: ETB hexproof-until-end-of-turn one-time grant. Durable +1/+2 below.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            1,
            2,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
