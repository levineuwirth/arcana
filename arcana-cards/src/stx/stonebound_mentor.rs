//! Stonebound Mentor — `{1}{R}{W}` 3/3 red/white Spirit Advisor. "Whenever
//! one or more cards leave your graveyard, scry 1."
//!
//! GAP: trigger — no TriggerCondition for "cards leave graveyard". Best-effort
//! using ZoneChange from graveyard as proxy.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stonebound Mentor");
    let spirit = reg.interner_mut().intern("Spirit");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — no TriggerCondition for cards leaving graveyard;
                // using ZoneChange from graveyard as closest approximation
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new().controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Graveyard(0)),
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: on_card_leaves_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_card_leaves_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Scry { player: trig.controller, count: 1 }]
}
