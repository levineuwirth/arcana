//! Cockatrice — `{3}{G}{G}` 2/4 green Cockatrice with Flying.
//! "Whenever this creature blocks or becomes blocked by a non-Wall
//! creature, destroy that creature at end of combat."
//!
//! Abilities:
//! 1. Flying (keyword).
//! 2. SelfBlocksOrBecomesBlocked → schedule the destruction of the
//!    other combatant at the next end step (modelled via
//!    DelayedAction; the "non-Wall" restriction is a documented gap).

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Cockatrice");
    let cockatrice = reg.interner_mut().intern("Cockatrice");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cockatrice);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfBlocksOrBecomesBlocked,
            intervening_if: None,
            effect: destroy_other_at_end_of_combat,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn destroy_other_at_end_of_combat(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "non-Wall creature" restriction (don't fire vs Walls)
    // isn't expressible in the SelfBlocksOrBecomesBlocked trigger
    // filter surface; we destroy whatever the other combatant is.
    let Some(other) = trig.other_combatant() else {
        return Vec::new();
    };
    // GAP: "destroy that creature at end of combat" — DelayedWhen has
    // no EndCombat variant; schedule at the next end step (NextEndStep)
    // as the closest available timing.
    vec![Effect::DelayedAction {
        source: other,
        controller: trig.controller,
        when: DelayedWhen::NextEndStep,
        action: DelayedAction::Sacrifice,
    }]
}
