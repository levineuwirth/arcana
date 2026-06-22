//! Narcomoeba — `{1}{U}` 1/1 Illusion with Flying.
//!
//! Oracle:
//! * Flying — base keyword.
//! * "When this card is put into your graveyard from your library, you
//!   may put it onto the battlefield." — a library→graveyard
//!   ZoneChange trigger that returns this card to the battlefield. The
//!   "you may" is treated as resolution-time (always returns).

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Narcomoeba");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);

    let from_library_filter =
        ObjectFilter::creature().controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: from_library_filter,
                from: Some(Zone::Library(0)),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: return_self_to_battlefield,
            trigger_zones: vec![Zone::Library(0), Zone::Graveyard(0)],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn return_self_to_battlefield(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "You may put it onto the battlefield" — return this card (now in
    // the graveyard) to the battlefield.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: trig.source }]
}
