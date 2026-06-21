//! Consuming Aberration — `{3}{U}{B}` */* Horror.
//! "Consuming Aberration's power and toughness are each equal to the
//! number of cards in your opponents' graveyards."
//! "Whenever you cast a spell, each opponent reveals cards from the top
//! of their library until they reveal a land card, then puts those cards
//! into their graveyard."
//!
//! The */* P/T (PtValue::Star) is backed by a CDA counting opponents'
//! graveyard cards, which the demonstrated API can't install — GAP'd.
//! The SpellCast mill-until-land trigger is kept in shape, but its body
//! is GAP'd: RevealUntil routes the FOUND card only to hand/battlefield
//! (RevealDest), with no found→graveyard option, so the
//! everything-into-graveyard self-mill is inexpressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Consuming Aberration");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    // GAP: "power and toughness are each equal to the number of cards in your
    // opponents' graveyards" — the */* defining static (a CDA) is not
    // installable via the demonstrated API; PtValue::Star marks the slot.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: opponents_mill_until_land,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn opponents_mill_until_land(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each opponent reveals cards from the top of their library until they
    // reveal a land card, then puts those cards into their graveyard." —
    // RevealUntil's found card can only go to hand/battlefield (RevealDest), not
    // the graveyard, so routing the entire revealed run (land included) into the
    // graveyard is inexpressible.
    Vec::new()
}
