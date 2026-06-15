//! Airtight Alibi — `{2}{G}` enchantment — Aura.
//! "Flash. Enchant creature. When this Aura enters, untap enchanted creature.
//!  It gains hexproof until end of turn. If it's suspected, it's no longer
//!  suspected. Enchanted creature gets +2/+2 and can't become suspected."
//!
//! Flash buff Aura. The durable +2/+2 is the standard `attached_pt` install.
//! The one-time ETB effects (untap, hexproof-until-EOT, clear-suspected) and
//! the "can't become suspected" static are not expressible with the shown
//! attached-grant API.
//!
//! GAP: ETB untap + hexproof-until-end-of-turn + clear-suspected, and the
//! "can't become suspected" replacement — no shown builders.

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
use arcana_core::effects::KeywordAbility;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Airtight Alibi");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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

fn etb_install(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: ETB untap + hexproof-until-EOT + clear-suspected, and
    // "can't become suspected" static — not expressible. Durable +2/+2 below.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            2,
            2,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
