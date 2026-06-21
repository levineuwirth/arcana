//! Yotian Courier — `{U}{R}` 2/2 red-blue Human Artificer.
//!
//! Oracle:
//! * Flying
//! * "Whenever Yotian Courier attacks, choose one that wasn't chosen
//!   during your last combat —
//!     • Create a tapped Powerstone token.
//!     • Seek a nonland card with mana value equal to the number of
//!       Powerstones you control."
//!
//! Flying is wired. The attack trigger is a MODAL triggered ability
//! with a "not chosen last combat" gate; triggered abilities have no
//! modal-choice machinery here, the "tapped" Powerstone rider and the
//! last-combat tracking are unmodeled, and "Seek" has no Effect
//! variant — so the modal body is GAP'd while the SelfAttacks trigger
//! itself is registered.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yotian Courier");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_modal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_modal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal triggered ability ("choose one that wasn't chosen
    // during your last combat") — no modal-choice machinery on triggers,
    // the "tapped" Powerstone rider and last-combat tracking are
    // unmodeled, and Seek has no Effect variant.
    Vec::new()
}
