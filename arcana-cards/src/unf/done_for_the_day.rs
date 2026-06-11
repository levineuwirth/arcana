//! Done for the Day — `{2}{G}` enchantment (Unfinity).
//! "At the beginning of your end step, if you control an Employee, a
//! Performer, or a Robot, you may get {TK} or create a Treasure token. If
//! you control all three, you may put a sticker on a nonland permanent you
//! own."
//!
//! End-step trigger with an intervening-if over the three subtypes (OR).
//! The modelable arm — create a Treasure token — is emitted; tickets
//! ({TK}) and stickers are Un-set subsystems with no engine model.

use arcana_core::conditions;
use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Done for the Day");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_control_emp_perf_or_robot),
                effect: end_of_shift,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…if you control an Employee, a Performer, or a Robot…"
fn if_control_emp_perf_or_robot(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    conditions::you_control_a(s, you, &script::subtype_filter(reg, "Employee"))
        || conditions::you_control_a(
            s,
            you,
            &script::subtype_filter(reg, "Performer"),
        )
        || conditions::you_control_a(
            s,
            you,
            &script::subtype_filter(reg, "Robot"),
        )
}

/// "…you may get {TK} or create a Treasure token. If you control all three,
/// you may put a sticker on a nonland permanent you own."
fn end_of_shift(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: tickets ({TK}) and stickers are Un-set subsystems with no engine
    // model, and the may-choice between the two modes is not expressible —
    // the modelable arm (create a Treasure token) is emitted
    // unconditionally.
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }]
}
