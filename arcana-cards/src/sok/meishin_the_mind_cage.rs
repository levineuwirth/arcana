//! Meishin, the Mind Cage — `{4}{U}{U}{U}` Legendary Enchantment.
//! "All creatures get -X/-0, where X is the number of cards in your
//! hand."
//!
//! Implementation: an ETB trigger installs a GLOBAL DYNAMIC filtered
//! pump over ALL creatures (no controller scope), with a `compute` fn
//! that reads the source controller's hand size and returns
//! `(-X, 0)`. The buff is re-evaluated every layer pass and auto-
//! expires when this enchantment leaves the battlefield. The `compute`
//! reads only a state scalar (hand size) — never a layer-computed P/T —
//! so it is recursion-proof.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Meishin, the Mind Cage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
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

/// ETB trigger: install "all creatures get -X/-0, where X is the number
/// of cards in your hand".
fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump_dynamic(
            trig.source,
            ObjectFilter::creature(),
            compute_hand_penalty,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// X = cards in the source controller's hand; the pump is -X/-0. Reads
/// only the hand-size scalar — recursion-safe.
fn compute_hand_penalty(state: &GameState, source: ObjectId) -> (i32, i32) {
    let controller = state.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let x = script::hand_size(state, controller) as i32;
    (-x, 0)
}
