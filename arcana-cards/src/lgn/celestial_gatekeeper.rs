//! Celestial Gatekeeper — `{3}{W}{W}` 2/2 Bird Cleric with Flying.
//!
//! "When this creature dies, exile it, then return up to two target
//! Bird and/or Cleric permanent cards from your graveyard to the
//! battlefield."
//!
//! Flying is a base keyword. The dies trigger returns up to two
//! Bird/Cleric cards from your graveyard to the battlefield. The
//! "exile it" self-exile rider is GAP'd (no self-exile-from-graveyard
//! primitive in this trigger surface).

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
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
    let name = reg.interner_mut().intern("Celestial Gatekeeper");
    let bird = reg.interner_mut().intern("Bird");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(cleric);

    // Bird and/or Cleric permanent cards in your graveyard.
    let gy_filter = ObjectFilter::new()
        .with_subtypes_any(vec![bird, cleric])
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_reanimate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: gy_filter,
                    },
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
            }),
    )
}

fn dies_reanimate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile it" self-exile rider (no self-exile-from-graveyard
    // primitive). Return up to two chosen Bird/Cleric cards.
    trig.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => {
                Some(Effect::ReturnFromGraveyardToBattlefield { target: *id })
            }
            _ => None,
        })
        .collect()
}
