//! Indrik Umbra — `{4}{G}{W}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +4/+4 and has first strike,
//!  and all creatures able to block it do so. Umbra armor (If enchanted
//!  creature would be destroyed, instead remove all damage from it and
//!  destroy this Aura.)"
//!
//! Installs +4/+4 (`attached_pt`) and first strike (`attached_keyword`).
//! GAP: "all creatures able to block it do so" (lure) has no attached
//! must-be-blocked builder, and Umbra armor (a destroy-replacement that
//! sacrifices this Aura instead) has no attached replacement-effect
//! builder.

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
    let name = reg.interner_mut().intern("Indrik Umbra");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
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
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "all creatures able to block it do so" (lure) and Umbra armor
    // (destroy-replacement that sacrifices this Aura instead) — no builders.
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt(
                trig.source,
                4,
                4,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::FirstStrike,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
