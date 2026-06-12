//! Urborg Stalker — `{3}{B}` 2/4 black creature. "At the beginning of each
//! player's upkeep, if that player controls a nonblack, nonland permanent, this
//! creature deals 1 damage to that player."
//!
//! "That player" is the upkeep owner — the active player while the trigger
//! fires/resolves (`state.active_player()`); the intervening-if checks that
//! player for a nonblack, nonland permanent.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urborg Stalker");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(if_upkeep_player_has_nonblack_nonland),
                effect: upkeep_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "…if that player controls a nonblack, nonland permanent…"
fn if_upkeep_player_has_nonblack_nonland(
    s: &GameState,
    _src: ObjectId,
    _you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    // "That player" = whose upkeep it is = the active player.
    let them = s.active_player();
    !script::ids_matching(
        s,
        &ObjectFilter::permanent()
            .without_colors(ColorSet::black())
            .without_types(TypeLine::LAND.into())
            .controlled_by(ControllerConstraint::You),
        them,
    )
    .is_empty()
}

fn upkeep_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "That player" = whose upkeep it is = the active player.
    vec![Effect::DealDamage {
        target: DamageTarget::Player(state.active_player()),
        amount: 1,
        source: trig.source,
    }]
}
