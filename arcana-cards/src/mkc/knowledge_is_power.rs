//! Knowledge Is Power — `{3}{W}{U}` enchantment. "Creatures you control
//! get +X/+X, where X is the number of cards you've drawn this turn."
//!
//! Implementation: an ETB trigger installs a GLOBAL DYNAMIC filtered
//! pump over the creatures the source's controller controls, with a
//! `compute` fn that reads the scalar "cards drawn this turn" for that
//! controller and returns `(X, X)`. The buff is re-evaluated every
//! layer pass and auto-expires when this enchantment leaves the
//! battlefield (`Duration::WhileSourceOnBattlefield`). The `compute`
//! reads only a state scalar (never a layer-computed P/T), so it is
//! recursion-proof.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knowledge Is Power");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
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

/// ETB trigger: install "creatures you control get +X/+X, where X is
/// the number of cards you've drawn this turn".
fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump_dynamic(
            trig.source,
            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            compute_drawn,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// X = cards the source's controller has drawn this turn; the pump is
/// +X/+X. Reads only the "cards drawn this turn" scalar — recursion-safe.
fn compute_drawn(state: &GameState, source: ObjectId) -> (i32, i32) {
    let controller = state.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let x = script::cards_drawn_this_turn(state, controller) as i32;
    (x, x)
}
