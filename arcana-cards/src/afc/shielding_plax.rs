//! Shielding Plax — `{2}{G/U}` enchantment — Aura.
//! "Enchant creature. When this Aura enters, draw a card. Enchanted
//! creature can't be the target of spells or abilities your opponents
//! control."
//!
//! ETB draws a card for the Aura's controller and installs Hexproof on
//! the host ("can't be the target of spells or abilities your opponents
//! control" is precisely Hexproof). The `{G/U}` hybrid makes the card
//! both green and blue.

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
    let name = reg.interner_mut().intern("Shielding Plax");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G/U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
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
                effect: etb_draw_and_grant,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_draw_and_grant(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let controller = state
        .objects
        .get(trig.source)
        .map(|o| o.controller)
        .unwrap_or(0);
    vec![
        Effect::DrawCards { player: controller, count: 1 },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Hexproof,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
