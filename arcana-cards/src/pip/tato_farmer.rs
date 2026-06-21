//! Tato Farmer — `{2}{G}` 1/4 Creature — Zombie Mutant Peasant.
//! "Landfall — Whenever a land you control enters, you may get two rad counters.
//!  {T}: Put target land card in a graveyard that was milled this turn onto the
//!  battlefield under your control tapped."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tato Farmer");
    let zombie = reg.interner_mut().intern("Zombie");
    let mutant = reg.interner_mut().intern("Mutant");
    let peasant = reg.interner_mut().intern("Peasant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(mutant);
    subtypes.0.insert(peasant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // Landfall: whenever a land you control enters.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: landfall_rad,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Put target land card in a graveyard that was milled this turn onto the battlefield under your control tapped.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: reanimate_milled_land,
            }),
    )
}

fn landfall_rad(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may get two rad counters" — rad counters are placed on a player
    // and there is no player-counter / rad-counter effect in the documented
    // surface (Effect::AddCounters targets an ObjectId only).
    Vec::new()
}

fn reanimate_milled_land(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the target restriction "a land card in a graveyard that was milled
    // this turn" is not expressible (no milled-this-turn card filter), and there
    // is no documented effect to put a chosen graveyard card onto the battlefield
    // under YOUR control tapped (ReturnFromGraveyardToBattlefield returns to the
    // owner's control, untapped).
    Vec::new()
}
