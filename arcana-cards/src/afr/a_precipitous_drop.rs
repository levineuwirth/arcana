//! A-Precipitous Drop — `{1}{B}` enchantment — Aura.
//! "Enchant creature. When Precipitous Drop enters, venture into the dungeon.
//! Enchanted creature gets -2/-2. It gets -5/-5 instead as long as you've
//! completed a dungeon."
//!
//! ETB ventures into the dungeon and installs the base -2/-2 debuff.
//! GAP: the conditional upgrade to -5/-5 "as long as you've completed a
//! dungeon" is not expressible (no completed-dungeon static condition for an
//! attached P/T); the base -2/-2 is always applied.

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
    let name = reg.interner_mut().intern("A-Precipitous Drop");
    let aura = reg.interner_mut().intern("Aura");
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
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_venture_and_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_venture_and_install(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the -5/-5 "as long as you've completed a dungeon" upgrade is not
    // expressible; the base -2/-2 is always applied.
    let controller = state
        .objects
        .get(trig.source)
        .map(|o| o.controller)
        .unwrap_or(0);
    vec![
        Effect::Venture { player: controller },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt(
                trig.source,
                -2,
                -2,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
