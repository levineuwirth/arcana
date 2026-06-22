//! Wall of Tears — `{1}{U}` 0/4 Wall (blue) with Defender.
//!
//! Oracle:
//! * "Defender" — keyword.
//! * "Whenever this creature blocks a creature, return that creature to its
//!   owner's hand at end of combat." — SelfBlocks trigger; the blocked creature
//!   (`trig.other_combatant()`) is scheduled to be returned to its owner's hand.
//!   Fidelity GAP: there is no `DelayedWhen::EndOfCombat`, so the return is
//!   scheduled for the next end step (slightly later than "end of combat", but
//!   the creature is bounced).

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
    let name = reg.interner_mut().intern("Wall of Tears");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBlocks,
                intervening_if: None,
                effect: bounce_blocked,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn bounce_blocked(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.other_combatant() else { return Vec::new(); };
    vec![Effect::DelayedAction {
        source: id,
        controller: trig.controller,
        when: DelayedWhen::NextEndStep,
        action: DelayedAction::ReturnToHand,
    }]
}
