//! Eye of Nidhogg — `{2}{B}` legendary enchantment — Aura.
//! "Enchant creature. Enchanted creature is a black Dragon with base power and
//!  toughness 4/2, has flying and deathtouch, and is goaded. When Eye of
//!  Nidhogg is put into a graveyard from the battlefield, return it to its
//!  owner's hand."
//!
//! The animation grant is a bundle of ETB-installed continuous effects:
//! `attached_set_pt(4,2)` + `attached_colors(black)` + `attached_subtypes
//! (Dragon)` + `attached_keyword(Flying)` + `attached_keyword(Deathtouch)`.
//! "Is goaded" has no goad primitive and is GAP'd. A `SelfDies` trigger on the
//! Aura itself returns this card to its owner's hand.

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
use arcana_core::types::{CardId, ColorSet, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eye of Nidhogg");
    let aura = reg.interner_mut().intern("Aura");
    let _dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
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
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_self_dies,
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
    let mut dragon_set = SubtypeSet::default();
    if let Some(d) = reg.interner().lookup("Dragon") {
        dragon_set.0.insert(d);
    }
    // GAP: "is goaded" — no goad primitive expressible here.
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_set_pt(
                trig.source,
                4,
                2,
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
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_subtypes(
                trig.source,
                dragon_set,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Flying,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Deathtouch,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}

fn on_self_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnToHand { target: trig.source }]
}
