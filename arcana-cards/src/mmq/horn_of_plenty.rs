//! Horn of Plenty — `{6}` artifact (Mercadian Masques, 1999).
//! "Whenever a player casts a spell, they may pay {1}. If the player does,
//! they draw a card at the beginning of the next end step."
//!
//! An any-caster `SpellCast` trigger; the caster is read via
//! `trig.triggering_caster()` and taxed with `Effect::OptionalPayment`.
//! The draw should be delayed to the next end step — `DelayedAction` has no
//! Draw action, so the draw is immediate (documented GAP).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Horn of Plenty");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: offer_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…they may pay {1}. If the player does, they draw a card at the
/// beginning of the next end step."
fn offer_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(caster) = trig.triggering_caster() else {
        return Vec::new();
    };
    // GAP: the draw should happen at the beginning of the next end step;
    // DelayedAction has no Draw action, so the draw resolves immediately.
    vec![Effect::OptionalPayment {
        chooser: caster,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{1}").expect("valid cost"),
        ),
        then: Box::new(Effect::DrawCards {
            player: caster,
            count: 1,
        }),
        else_effect: None,
    }]
}
