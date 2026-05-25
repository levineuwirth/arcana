//! Michiko Konda, Truth Seeker — `{3}{W}` 2/2 white Legendary Creature —
//! Human Advisor.
//! "Whenever a source an opponent controls deals damage to you, that player
//! sacrifices a permanent of their choice."
//! GAP: trigger — "a source an opponent controls deals damage to you" maps
//! closest to DamageDealt with source_filter controlled by Opponent and
//! target_filter Player. Using that approximation.
//! GAP: effect — "sacrifices a permanent of their choice" (opponent chooses
//! which permanent to sacrifice) — Sacrifice effect uses ObjectFilter; there
//! is no "controller chooses permanent" variant. Using permanent filter as
//! best effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Michiko Konda, Truth Seeker");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new()
                        .controlled_by(ControllerConstraint::Opponent),
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                intervening_if: None,
                effect: on_damage_sac_permanent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_damage_sac_permanent(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "that player" (the opponent who controlled the source) sacrifices a permanent.
    // GAP: the triggering caster is an approximation for the opponent controller.
    let Some(p) = trig.triggering_caster() else { return Vec::new(); };
    vec![Effect::Sacrifice {
        player: p,
        filter: ObjectFilter::new(),
        count: 1,
    }]
}
