//! Junk Winder — `{5}{U}{U}` 5/6 blue Serpent.
//!
//! Oracle:
//! * Affinity for tokens (cost reduction — GAP, no usable keyword)
//! * "Whenever a token you control enters, tap target nonland
//!   permanent an opponent controls. It doesn't untap during its
//!   controller's next untap step."
//!
//! The token-enters trigger taps a chosen opponent's nonland
//! permanent. The "doesn't untap during its controller's next untap
//! step" rider has no no-untap duration in the catalog, so it is
//! GAP'd (Tap is applied; the skipped untap is omitted).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Junk Winder");
    let serpent = reg.interner_mut().intern("Serpent");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);

    // GAP: "Affinity for tokens" — cost reduction; no usable keyword
    // variant for this card class.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::permanent()
                    .controlled_by(ControllerConstraint::You)
                    .tokens_only(),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: tap_opponent_nonland,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .controlled_by(ControllerConstraint::Opponent)
                        .without_types(TypeLine::LAND.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn tap_opponent_nonland(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "It doesn't untap during its controller's next untap step."
    // — no no-untap duration primitive in the catalog.
    vec![Effect::Tap { target: *id }]
}
