//! Clawing Torment — `{B}` enchantment — Aura.
//! "Enchant artifact or creature. As long as enchanted permanent is a creature,
//!  it gets -1/-1 and can't block. Enchanted permanent has \"At the beginning of
//!  your upkeep, you lose 1 life.\""
//!
//! Best-effort: ETB-installs `attached_pt(-1/-1)` + `attached_cant_block`
//! (the "as long as it's a creature" gate is honored in practice when the host
//! is a creature). The granted upkeep "you lose 1 life" triggered ability is
//! not expressible (no granted-triggered-ability builder) and is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Clawing Torment");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        // NOTE: artifact-or-creature widened to any permanent (no disjunction filter).
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Permanent(ObjectFilter::permanent()))
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
    // GAP: granted "At the beginning of your upkeep, you lose 1 life" triggered
    // ability has no builder. The -1/-1 + can't-block carries the "as long as
    // creature" clause via the practical assumption that the host is a creature.
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt(
                trig.source,
                -1,
                -1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_cant_block(
                trig.source,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
