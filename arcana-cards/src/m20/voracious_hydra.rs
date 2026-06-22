//! Voracious Hydra — `{X}{G}{G}` 0/1 Hydra with Trample.
//!
//! Oracle:
//! * Trample — keyword, base characteristic. (Scryfall also parses "Fight",
//!   which is not a `KeywordAbility` variant — it's part of the modal text.)
//! * "This creature enters with X +1/+1 counters on it." — GAP: an "enters with
//!   X counters" replacement where X is the spell's chosen value has no primitive
//!   (a triggered ETB body cannot read X).
//! * "When this creature enters, choose one —
//!     • Double the number of +1/+1 counters on this creature.
//!     • This creature fights target creature you don't control." — GAP: modal
//!   selection ("choose one") is only wired for spell abilities; a triggered
//!   ability's effect fn has no mode-choice hook. The trigger is wired with the
//!   mode-1 fight target requirement so the shape is recorded, but emitting one
//!   fixed mode would be materially wrong, so the body returns `Vec::new()`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Voracious Hydra");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_modal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn etb_modal(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: modal "choose one" ETB. Modal selection is only wired for spell
    //      abilities (ModalSpec / with_mode_effects); a triggered ability's
    //      effect fn has no mode-choice hook, so neither "double the +1/+1
    //      counters" nor "fight target creature you don't control" can be
    //      offered as a choice. Emitting one fixed mode would be materially
    //      wrong.
    Vec::new()
}
