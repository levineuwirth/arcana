//! Living Terrain — `{2}{G}{G}` enchantment — Aura.
//! "Enchant land. Enchanted land is a 5/6 green Treefolk creature that's
//! still a land."
//!
//! A land-animation Aura. ETB installs the bundle: attached_set_pt sets
//! base 5/6, attached_types adds the creature type, attached_subtypes
//! adds Treefolk, and attached_colors makes it green. The "still a land"
//! clause is preserved because the type-add is additive.

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
    let name = reg.interner_mut().intern("Living Terrain");
    let aura = reg.interner_mut().intern("Aura");
    let _treefolk = reg.interner_mut().intern("Treefolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
            }),
    )
}

fn etb_install(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let mut treefolk = SubtypeSet::default();
    if let Some(t) = reg.interner().lookup("Treefolk") {
        treefolk.0.insert(t);
    }
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_set_pt(
                trig.source,
                5,
                6,
                Duration::WhileSourceOnBattlefield,
            ),
        },
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
                treefolk,
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
    ]
}
