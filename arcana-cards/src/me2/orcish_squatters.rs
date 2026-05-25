//! Orcish Squatters — `{4}{R}` 2/3 red creature. "Whenever this creature attacks
//! and isn't blocked, you may gain control of target land defending player
//! controls for as long as you control this creature. If you do, this creature
//! assigns no combat damage this turn."
//!
//! GAP: effect — "for as long as you control this creature" duration not
//! expressible (ChangeControl is permanent, ChangeControlEot is end of turn
//! only). Emitting ChangeControl as best-effort; "assigns no combat damage" rider
//! GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Orcish Squatters");
    let orc = reg.interner_mut().intern("Orc");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacksUnblocked,
                intervening_if: None,
                effect: unblocked_steal_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::LAND.into())
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn unblocked_steal_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: effect — "for as long as you control this creature" duration; ChangeControl is permanent
    // GAP: effect — "assigns no combat damage this turn" rider
    vec![Effect::ChangeControl { target: *id, new_controller: trig.controller }]
}
