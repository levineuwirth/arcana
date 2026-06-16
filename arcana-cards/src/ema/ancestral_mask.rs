//! Ancestral Mask — `{2}{G}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +2/+2 for each other
//!  enchantment on the battlefield."
//!
//! The dynamic buff is an `attached_pt_dynamic` whose compute fn counts
//! every enchantment on the battlefield (all controllers), then subtracts
//! one for "each OTHER enchantment" (the Aura itself is excluded), giving
//! +2/+2 per other enchantment. The filter is type-only (no named subtype),
//! so it is fully expressible.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ancestral Mask");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
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

fn etb_install(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt_dynamic(
            trig.source,
            other_enchantment_pump,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn other_enchantment_pump(state: &GameState, source: ObjectId) -> (i32, i32) {
    let Some(controller) = state.object_or_lki(source).map(|o| o.controller) else {
        return (0, 0);
    };
    let filter = ObjectFilter::permanent().with_types(TypeLine::ENCHANTMENT.into());
    // Count all enchantments on the battlefield, then subtract one (this Aura)
    // for "each OTHER enchantment".
    let total = script::count_matching(state, &filter, controller) as i32;
    let others = (total - 1).max(0);
    (2 * others, 2 * others)
}
