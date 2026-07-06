//! Breathless Knight — `{1}{W}{B}` 2/2 Spirit Knight with Flying and Lifelink.
//!
//! Oracle:
//!  * Flying, lifelink.
//!  * Whenever this creature or another creature you control enters, if that
//!    creature entered from a graveyard or you cast it from a graveyard, put a
//!    +1/+1 counter on this creature.
//!
//! Keywords are wired. The trigger fires on a creature you control entering,
//! but its intervening-if ("entered from a graveyard / you cast it from a
//! graveyard") has no condition helper. Firing unconditionally would be
//! materially wrong, so the effect is GAP'd while the trigger shape is recorded.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Breathless Knight");
    let spirit = reg.interner_mut().intern("Spirit");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // "if that creature entered from a graveyard" — gate the trigger on the
            // graveyard origin (same_kind matches any player's graveyard). The
            // "or you cast it from a graveyard" alternative enters from the stack
            // and stays a GAP.
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                from: Some(Zone::Graveyard(0)),
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: etb_graveyard_counter,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_graveyard_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // The trigger is gated on the graveyard origin (from: Graveyard), so put the
    // +1/+1 counter on Breathless Knight itself whenever it fires.
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;
    use arcana_core::events::{GameEvent, MoveCause};

    #[test]
    fn a_creature_entering_from_graveyard_puts_a_counter_on_self() {
        // The trigger is gated on the graveyard origin (from: Graveyard); when it
        // fires the effect must put a +1/+1 counter on Breathless Knight (source),
        // not silently no-op as it did while GAP'd.
        let reg = CardRegistry::new();
        let s = GameState::new(2, 0);
        let trig = PendingTrigger {
            source: 5,
            trigger_id: 1,
            controller: 0,
            trigger_event: GameEvent::ZoneChange {
                object_id: 1,
                from: Zone::Graveyard(0),
                to: Zone::Battlefield,
                new_id: 2,
                cause: MoveCause::StateBasedAction,
            },
            targets: Default::default(),
            effect_override: None,
        };
        match etb_graveyard_counter(&s, &trig, &reg).as_slice() {
            [Effect::AddCounters { target: 5, kind: CounterKind::PlusOnePlusOne, count: 1 }] => {}
            other => panic!("expected a single +1/+1 counter on self, got {other:?}"),
        }
    }
}
