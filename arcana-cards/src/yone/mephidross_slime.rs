//! Mephidross Slime — `{1}{B}{G}` 3/3 Phyrexian Ooze.
//!
//! Trample.
//! When Mephidross Slime dies, conjure a card named Mephidross Slime into your
//! graveyard. Perpetually double the power and toughness of Mephidross Slime
//! and that card, then shuffle them into their owner's library.
//!
//! The dies trigger is GAP'd: Conjure is an Arena-only mechanic with no
//! `Effect::Conjure` variant, and "perpetually double power/toughness" plus
//! shuffle-into-library are likewise unexpressible. The trigger fires but its
//! effect is empty.

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
    let name = reg.interner_mut().intern("Mephidross Slime");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let ooze = reg.interner_mut().intern("Ooze");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(ooze);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: dies_conjure,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dies_conjure(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Conjure (Arena-only, no Effect::Conjure) + perpetually double P/T +
    // shuffle into library — none expressible.
    Vec::new()
}
