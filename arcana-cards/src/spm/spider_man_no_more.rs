//! Spider-Man No More — `{1}{U}` enchantment — Aura.
//! "Enchant creature. Enchanted creature is a Citizen with base power and
//!  toughness 1/1. It has defender and loses all other abilities. (It also
//!  loses all other creature types.)"
//!
//! Partial: the Citizen subtype add and the defender grant are ETB-installed.
//! The base-P/T setting (1/1), "loses all other abilities", and "loses all
//! other creature types" are not expressible (attached_pt is additive only;
//! no ability-/type-removal grant) — GAP.

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
    let name = reg.interner_mut().intern("Spider-Man No More");
    let aura = reg.interner_mut().intern("Aura");
    let _citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
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

fn etb_install(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: base-P/T set to 1/1 (attached_pt is additive only); "loses all
    // other abilities" and "loses all other creature types" — no removal
    // grant builder.
    let citizen = reg
        .interner()
        .lookup("Citizen")
        .expect("Citizen interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(citizen);
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_subtypes(
                trig.source,
                subtypes,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Defender,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
