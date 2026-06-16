//! Noggle the Mind — `{1}{U}` enchantment — Aura.
//! "Flash. Enchant creature. Enchanted creature loses all abilities and is a
//!  colorless Noggle with base power and toughness 1/1."
//!
//! Flash is a card keyword. The base-1/1 set is `attached_set_pt` and the
//! Noggle type is `attached_subtypes`. "Loses all abilities" and "becomes
//! colorless" (color REMOVAL — `attached_colors` is additive only) have no
//! expressible primitives — GAP those.

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
    let name = reg.interner_mut().intern("Noggle the Mind");
    let aura = reg.interner_mut().intern("Aura");
    let _noggle = reg.interner_mut().intern("Noggle");
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
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "loses all abilities" (no abilities-removal primitive) and "is
    // colorless" (attached_colors is additive — no color removal).
    let mut noggle_set = SubtypeSet::default();
    if let Some(sym) = reg.interner().lookup("Noggle") {
        noggle_set.0.insert(sym);
    }
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_set_pt(
                trig.source,
                1,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_subtypes(
                trig.source,
                noggle_set,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
