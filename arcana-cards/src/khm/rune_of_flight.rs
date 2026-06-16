//! Rune of Flight — `{1}{U}` enchantment — Aura Rune.
//! "Enchant permanent. When this Aura enters, draw a card. As long as
//!  enchanted permanent is a creature, it has flying. As long as enchanted
//!  permanent is an Equipment, it has 'Equipped creature has flying.'"
//!
//! ETB "draw a card" is expressible (DrawCards). The "as long as it's a
//! creature, it has flying" clause is approximated by an unconditional
//! `attached_keyword(Flying)` (inert on non-creatures for combat purposes).
//! The "as long as it's an Equipment, it has 'Equipped creature has
//! flying'" nested conditional grant is not expressible — GAP for that
//! clause.

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
    let name = reg.interner_mut().intern("Rune of Flight");
    let aura = reg.interner_mut().intern("Aura");
    let rune = reg.interner_mut().intern("Rune");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    subtypes.0.insert(rune);
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

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "as long as enchanted permanent is an Equipment, it has 'Equipped
    // creature has flying'" — nested conditional grant not expressible.
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Flying,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
