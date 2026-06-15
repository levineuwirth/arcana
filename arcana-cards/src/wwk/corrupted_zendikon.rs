//! Corrupted Zendikon — `{1}{B}` enchantment — Aura.
//! "Enchant land. Enchanted land is a 3/3 black Ooze creature. It's still a
//!  land. When enchanted land dies, return that card to its owner's hand."
//!
//! Aura on a land. The engine attaches via `with_enchant`. We install the
//! ADDITIVE characteristic grants we can express: the Creature type
//! (`attached_types`), the Ooze subtype (`attached_subtypes`), and the black
//! color (`attached_colors`), all following `source.attached_to`. The host
//! dies-trigger returns the dying land card to its owner's hand.
//!
//! GAP: base-P/T-setting aura — "is a 3/3" sets base power/toughness;
//! `attached_pt` is additive only, so the 3/3 cannot be expressed.

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
    let name = reg.interner_mut().intern("Corrupted Zendikon");
    let aura = reg.interner_mut().intern("Aura");
    // Interned here so the ETB effect fn can `lookup` it.
    let _ooze = reg.interner_mut().intern("Ooze");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
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
                effect: etb_install_ooze_grants,
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
                effect: return_dying_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_ooze_grants(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let ooze = reg.interner().lookup("Ooze").expect("Ooze interned");
    let mut sub = SubtypeSet::default();
    sub.0.insert(ooze);
    // GAP: base-P/T-setting aura — "is a 3/3"; attached_pt is additive only,
    // cannot set base 3/3.
    vec![
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
                sub,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_colors(
                trig.source,
                ColorSet::black(),
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}

/// "When enchanted land dies, return that card to its owner's hand."
fn return_dying_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(dead) = trig.dying_object() else {
        return Vec::new();
    };
    vec![Effect::ReturnToHand { target: dead }]
}
