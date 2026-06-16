//! Ichthyomorphosis — `{2}{U}` enchantment — Aura.
//! "Enchant creature. Enchanted creature loses all abilities and is a blue
//!  Fish with base power and toughness 0/1."
//!
//! The "becomes a blue Fish 0/1" animation is faithfully installed:
//! `attached_set_pt(0, 1)` + `attached_colors(blue)` + `attached_subtypes(Fish)`.
//! "Loses all abilities" has no attached lose-all-abilities builder (only
//! specific keyword removal is supported).
//! GAP: "loses all abilities".

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
    let name = reg.interner_mut().intern("Ichthyomorphosis");
    let aura = reg.interner_mut().intern("Aura");
    // Interned here so the ETB effect fn can `lookup` it.
    let _fish = reg.interner_mut().intern("Fish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
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

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let fish = reg.interner().lookup("Fish").expect("Fish interned");
    let mut fish_set = SubtypeSet::default();
    fish_set.0.insert(fish);
    // GAP: "loses all abilities" — no attached lose-all-abilities builder.
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_set_pt(
                trig.source,
                0,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_colors(
                trig.source,
                ColorSet::blue(),
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_subtypes(
                trig.source,
                fish_set,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
