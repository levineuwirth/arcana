//! Eagles of the North — `{5}{W}` 3/3 white Bird Soldier with Flying.
//!
//! Oracle:
//! * Flying.
//! * When this creature enters, creatures you control get +1/+0 and gain
//!   first strike until end of turn.
//! * Plainscycling {1} ({1}, Discard this card: Search your library for a
//!   Plains card, ...).
//!
//! Flying is on the keyword line. Plainscycling is recorded as the generic
//! `Cycling({1})` keyword (the engine synthesizes the discard-to-draw
//! activated ability; the basic-land-type search variant degrades to plain
//! cycling — a documented fidelity gap). The ETB anthem pumps each creature
//! you control +1/+0 and grants first strike until end of turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eagles of the North");
    let bird = reg.interner_mut().intern("Bird");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Cycling(ManaCost::parse("{1}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_anthem,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Each creature you control gets +1/+0 and gains first strike until EOT.
fn etb_anthem(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    ids.into_iter()
        .map(|id| Effect::Pump {
            target: id,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::FirstStrike],
        })
        .collect()
}
