//! Sarinth Steelseeker — `{1}{G}` 1/2 green Human Artificer Scout.
//! "Whenever an artifact you control enters, look at the top card of
//! your library. If it's a land card, you may reveal it and put it into
//! your hand. If you don't put the card into your hand, you may put it
//! into your graveyard."
//!
//! Modeled as DigTopN { count: 1, filter: land card, rest: Graveyard }.
//! GAP: the "may put into graveyard" is deterministic in DigRest::Graveyard
//! (the card always goes to graveyard if not taken); the oracle's optional
//! graveyard placement is a fidelity gap.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sarinth Steelseeker");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::ARTIFACT.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: artifact_etb_dig_for_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn artifact_etb_dig_for_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 1,
        filter: Some(ObjectFilter {
            types: Some(TypeLine::LAND.into()),
            ..ObjectFilter::default()
        }),
        rest: DigRest::Graveyard,
    }]
}
