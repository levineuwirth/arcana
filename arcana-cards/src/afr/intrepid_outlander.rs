//! Intrepid Outlander — `{1}{G}` 2/3 green Creature — Orc Ranger.
//!
//! Reach
//! Pack tactics — Whenever this creature attacks, if you attacked with
//!   creatures with total power 6 or greater this combat, venture into the
//!   dungeon.
//!
//! Decomposition: Reach → `keywords`. The Pack-tactics attack trigger →
//! one `TriggeredAbilityDef` (`SelfAttacks`) emitting `Effect::Venture`.
//! The "Pack tactics" keyword itself is not on the supported surface; the
//! intervening-if "if you attacked with creatures with total power 6 or
//! greater this combat" has no available condition helper (no
//! total-attacking-power predicate), so it fires unconditionally — a
//! documented fidelity gap matching the Werewolf Pack Leader precedent.

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
    let name = reg.interner_mut().intern("Intrepid Outlander");
    let orc = reg.interner_mut().intern("Orc");
    let ranger = reg.interner_mut().intern("Ranger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(ranger);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            // GAP: intervening-if "if you attacked with creatures with total
            // power 6 or greater this combat" — no total-attacking-power
            // condition helper; fires unconditionally.
            intervening_if: None,
            effect: attacks_venture,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attacks_venture(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Venture {
        player: trig.controller,
    }]
}
