//! Kenrith's Transformation — `{1}{G}` enchantment — Aura.
//! "Enchant creature. When this Aura enters, draw a card. Enchanted
//!  creature loses all abilities and is a green Elk creature with base
//!  power and toughness 3/3."
//!
//! ETB draws a card. The animation portion is wired via attached_set_pt
//! (base 3/3), attached_colors (green), and attached_subtypes (Elk). The
//! "loses all abilities" / "loses all other card types and creature
//! types" clauses have no expressible builder, so they are GAP'd.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Kenrith's Transformation");
    let aura = reg.interner_mut().intern("Aura");
    let _elk = reg.interner_mut().intern("Elk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "loses all abilities" and "loses all other card types and
    // creature types" have no expressible attached_* builder; only the
    // draw, base 3/3, green color, and Elk subtype are wired.
    let mut elk_set = SubtypeSet::default();
    if let Some(e) = reg.interner().lookup("Elk") {
        elk_set.0.insert(e);
    }
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_set_pt(
                trig.source,
                3,
                3,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_colors(
                trig.source,
                ColorSet::green(),
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_subtypes(
                trig.source,
                elk_set,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
