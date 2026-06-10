//! Lavabrink Floodgates — `{3}{R}` artifact (Zendikar Rising, 2020).
//! "{T}: Add {R}{R}." and "At the beginning of each player's upkeep,
//! that player may put a doom counter on this artifact or remove a
//! doom counter from it. Then if it has three or more doom counters on
//! it, sacrifice this artifact. When you do, it deals 6 damage to each
//! creature." The mana ability is wired; the upkeep trigger is wired
//! with a GAP effect: the add-or-remove counter player choice is an
//! optional ACTION (OptionalPaymentKind covers only Mana / Life), and
//! the reflexive "when you do" sacrifice-then-sweep cannot be
//! composed.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lavabrink Floodgates");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {R}{R}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_two_red,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: doom_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_two_red(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Red, ctx.source),
            ManaUnit::plain(ManaColor::Red, ctx.source),
        ],
    }]
}

fn doom_counters(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "that player may put a doom counter on this artifact or
    // remove a doom counter from it" is an optional add-or-remove
    // ACTION choice (OptionalPaymentKind covers only Mana / Life), and
    // the reflexive "Then if it has three or more doom counters,
    // sacrifice this artifact. When you do, it deals 6 damage to each
    // creature" chain cannot be composed from the demonstrated
    // primitives.
    Vec::new()
}
