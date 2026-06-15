//! Guardian Zendikon — `{2}{W}` enchantment — Aura.
//! "Enchant land. Enchanted land is a 2/6 white Wall creature with defender.
//! It's still a land. When enchanted land dies, return that card to its
//! owner's hand."
//!
//! Animation Aura on a land. Expressible parts: ETB installs the added card
//! type (Creature), color (white), subtype (Wall), and keyword (Defender);
//! a second ability (`AttachedCreatureDoes(SelfDies)`) returns the dying land
//! to its owner's hand. GAP: the 2/6 BASE power/toughness — `attached_pt` is
//! additive only, so a base-P/T set is not expressible.

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
    let name = reg.interner_mut().intern("Guardian Zendikon");
    let aura = reg.interner_mut().intern("Aura");
    let _wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Permanent(
                ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
            ))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfDies),
                },
                intervening_if: None,
                effect: return_host,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: 2/6 base power/toughness set — attached_pt is additive only.
    let mut wall = SubtypeSet::default();
    if let Some(w) = reg.interner().lookup("Wall") {
        wall.0.insert(w);
    }
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_types(
                trig.source,
                TypeLine::CREATURE.into(),
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_colors(
                trig.source,
                ColorSet::white(),
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_subtypes(
                trig.source,
                wall,
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

fn return_host(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    let Some(host) = trig.dying_object() else {
        return Vec::new();
    };
    vec![Effect::ReturnToHand { target: host }]
}
