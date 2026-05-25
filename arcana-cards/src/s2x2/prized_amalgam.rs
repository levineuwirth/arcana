//! Prized Amalgam — `{1}{U}{B}` 3/3 blue-black Zombie.
//! "Whenever a creature enters, if it entered from your graveyard or
//! you cast it from your graveyard, return this card from your graveyard
//! to the battlefield tapped at the beginning of the next end step."
//! GAP: trigger condition — "if it entered from your graveyard or cast
//! from graveyard" has no exact variant; using ZoneChange for any creature
//! entering as closest approximation.
//! GAP: trigger fires only when this card is in graveyard (trigger_zones
//! includes Graveyard) — engine may not support graveyard triggers via
//! trigger_zones; included as best-effort.
//! GAP: "return this card from graveyard to battlefield tapped" at next end
//! step — DelayedAction on a graveyard card is not supported; emitting
//! ReturnFromGraveyardToBattlefield immediately as approximation.

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
    let name = reg.interner_mut().intern("Prized Amalgam");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
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
                // GAP: "if entered from graveyard or cast from graveyard" condition
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature(),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: on_creature_enters,
                trigger_zones: vec![Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_creature_enters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "at the beginning of the next end step, tapped" — approximated as
    // immediate return without tapped status
    vec![Effect::ReturnFromGraveyardToBattlefield { target: trig.source }]
}
