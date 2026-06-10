//! Minion Reflector — `{5}` artifact (Onslaught, 2002).
//! "Whenever a nontoken creature you control enters, you may pay {2}.
//! If you do, create a token that's a copy of that creature, except it
//! has haste and 'At the beginning of the end step, sacrifice this
//! permanent.'"
//!
//! Wired as a ZoneChange (battlefield-bound, nontoken creatures you
//! control) trigger whose effect wraps `Effect::CopyPermanent` in an
//! `Effect::OptionalPayment` on {2}. GAP: the token's "except" riders
//! (haste + sacrifice at the beginning of the end step) — the minted
//! copy's object id is not available to attach them.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Minion Reflector");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: reflect_entering_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you may pay {2}. If you do, create a token that's a copy of that
/// creature…"
fn reflect_entering_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.entering_object() else {
        return Vec::new();
    };
    // GAP: "except it has haste and 'At the beginning of the end step,
    // sacrifice this permanent.'" — the minted copy's object id is not
    // readable here, so the haste grant and the delayed sacrifice
    // cannot be attached to the token.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{2}").expect("valid cost"),
        ),
        then: Box::new(Effect::CopyPermanent { target: id }),
        else_effect: None,
    }]
}
