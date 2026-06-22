//! Aetherborn Marauder — `{3}{B}` 2/2 Aetherborn Rogue with Flying and
//! Lifelink.
//!
//! "When this creature enters, move any number of +1/+1 counters from other
//! permanents you control onto this creature."
//!
//! * Keyword line → Flying, Lifelink.
//! * ETB trigger: move +1/+1 counters from other permanents you control onto
//!   this creature, via one Effect::MoveCounter per source permanent (Spike
//!   Cannibal pattern).
//!
//! Fidelity note: "any number" is a player choice; this moves ALL +1/+1
//! counters from your other permanents (the established faithful approximation,
//! since there is no variable-count counter-move picker).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aetherborn Marauder");
    let aetherborn = reg.interner_mut().intern("Aetherborn");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aetherborn);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_move_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_move_counters(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::permanent().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    ids.into_iter()
        .filter(|&id| id != trig.source)
        .filter_map(|id| {
            let n = state
                .objects
                .get(id)
                .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
            if n == 0 {
                None
            } else {
                Some(Effect::MoveCounter {
                    from: id,
                    to: trig.source,
                    kind: CounterKind::PlusOnePlusOne,
                    count: n,
                })
            }
        })
        .collect()
}
