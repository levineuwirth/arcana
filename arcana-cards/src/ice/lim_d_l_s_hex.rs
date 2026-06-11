//! Lim-Dûl's Hex — `{1}{B}` enchantment (Ice Age, 1995).
//! "At the beginning of your upkeep, for each player, this enchantment
//! deals 1 damage to that player unless they pay {B} or {3}."
//!
//! An upkeep trigger that loops over all players, posting an
//! `Effect::OptionalPayment` per player with the 1 damage in
//! `else_effect`. GAP fidelity note: the payer's CHOICE between {B}
//! and {3} is not expressible — the universally payable {3} is used.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lim-Dûl's Hex");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: hex_each_player,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…for each player, this enchantment deals 1 damage to that player
/// unless they pay {B} or {3}." GAP: the {B}-or-{3} cost choice is
/// modeled as a flat {3} (OptionalPaymentKind carries one cost).
fn hex_each_player(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let effects = script::all_players(state)
        .into_iter()
        .map(|p| Effect::OptionalPayment {
            chooser: p,
            cost: OptionalPaymentKind::Mana(
                ManaCost::parse("{3}").expect("valid cost"),
            ),
            then: Box::new(Effect::Sequence(vec![])),
            else_effect: Some(Box::new(Effect::DealDamage {
                target: DamageTarget::Player(p),
                amount: 1,
                source: trig.source,
            })),
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
