//! Tempest Technique — `{3}{W}` enchantment — Aura.
//! "Storm. Enchant creature you control. Enchanted creature gets +1/+1 for
//!  each enchantment you control."
//!
//! The dynamic +1/+1-per-enchantment buff is an `attached_pt_dynamic` whose
//! compute fn counts the enchantments the Aura's controller controls (a
//! type-only filter, no named subtype) — fully expressible.
//!
//! NOTE: controller wording ("enchant creature you control") approximated by
//! caster's choice.
//! GAP: Storm (not in the usable keyword surface) — `keywords: vec![]`.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tempest Technique");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        keywords: vec![], // GAP: Storm not in the usable keyword surface.
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
            enchantment_count_pump,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn enchantment_count_pump(state: &GameState, source: ObjectId) -> (i32, i32) {
    let Some(controller) = state.object_or_lki(source).map(|o| o.controller) else {
        return (0, 0);
    };
    let filter = ObjectFilter::permanent()
        .with_types(TypeLine::ENCHANTMENT.into())
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &filter, controller) as i32;
    (n, n)
}
