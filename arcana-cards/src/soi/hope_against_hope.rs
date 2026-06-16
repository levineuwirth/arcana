//! Hope Against Hope — `{2}{W}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +1/+1 for each creature you
//!  control. As long as enchanted creature is a Human, it has first strike."
//!
//! Dynamic buff: an ETB-installed `attached_pt_dynamic` whose compute fn counts
//! creatures the Aura controller controls (a type-only filter, fully
//! expressible) and returns (X, X). The conditional first strike (only while
//! the enchanted creature is a Human) is a subtype-gated static — GAP.

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
    let name = reg.interner_mut().intern("Hope Against Hope");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
    // GAP: conditional first strike "as long as enchanted creature is a Human".
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt_dynamic(
            trig.source,
            creatures_you_control,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn creatures_you_control(state: &GameState, source: ObjectId) -> (i32, i32) {
    let Some(controller) = state.object_or_lki(source).map(|o| o.controller) else {
        return (0, 0);
    };
    let n = script::count_matching(state, &ObjectFilter::creature(), controller) as i32;
    (n, n)
}
