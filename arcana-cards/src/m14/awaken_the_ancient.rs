//! Awaken the Ancient — `{1}{R}{R}{R}` enchantment — Aura (Zendikar).
//! "Enchant Mountain. Enchanted Mountain is a 7/7 red Giant creature
//!  with haste. It's still a land."
//!
//! Land animation. Enchants a Mountain (land with the Mountain subtype).
//! The grant SETS base P/T 7/7 (`attached_set_pt`), adds the CREATURE type
//! (`attached_types`, additive — keeping the land type), adds the Giant
//! subtype (`attached_subtypes`), adds red (`attached_colors`, additive),
//! and grants haste (`attached_keyword`). All installed on the fixed ETB
//! trigger and following `source.attached_to`.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Awaken the Ancient");
    let aura = reg.interner_mut().intern("Aura");
    let mountain = reg.interner_mut().intern("Mountain");
    // Intern "Giant" so the ETB effect fn can resolve it via `lookup`.
    let _giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Permanent(
                ObjectFilter::permanent()
                    .with_types(TypeLine::LAND.into())
                    .with_subtype_sym(mountain),
            ))
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
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut giant_subtypes = SubtypeSet::default();
    if let Some(giant) = reg.interner().lookup("Giant") {
        giant_subtypes.0.insert(giant);
    }
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_set_pt(
                trig.source,
                7,
                7,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_types(
                trig.source,
                TypeLine::CREATURE.into(),
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_subtypes(
                trig.source,
                giant_subtypes,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_colors(
                trig.source,
                ColorSet::red(),
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Haste,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
