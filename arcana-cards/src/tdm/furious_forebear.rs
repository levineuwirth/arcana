//! Furious Forebear — `{1}{W}` 3/1 white Spirit Warrior creature.
//! "Whenever a creature you control dies while this card is in your graveyard,
//! you may pay {1}{W}. If you do, return this card from your graveyard to your hand."
//!
//! # Notes
//! The trigger zone is Graveyard (non-standard). The "may pay" cost and
//! conditional return-to-hand is modeled as ReturnFromGraveyardToHand on source.
//! GAP: trigger fires from graveyard zone, not battlefield — trigger_zones uses
//! Graveyard(0) as best approximation; engine may not support non-battlefield trigger zones.
//! GAP: optional payment of {1}{W} before the effect — no cost-gate Effect variant available.

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
    let name = reg.interner_mut().intern("Furious Forebear");
    let spirit = reg.interner_mut().intern("Spirit");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: return_from_graveyard,
                trigger_zones: vec![Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn return_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: optional payment of {1}{W} cost-gate before returning — no cost-gate Effect variant.
    vec![Effect::ReturnFromGraveyardToHand { target: trig.source }]
}
