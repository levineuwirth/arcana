//! Meadowboon — `{2}{W}{W}` 3/3 white Elemental.
//!
//! Oracle text:
//! * "When this creature leaves the battlefield, put a +1/+1 counter on
//!   each creature target player controls."
//! * "Evoke {3}{W}" — alternative casting cost with sacrifice-on-ETB.
//!
//! Implementation notes:
//! * The leaves-the-battlefield trigger has no dedicated `TriggerCondition`
//!   variant in the usable catalog; the closest is `SelfDies` (covers the
//!   battlefield→graveyard case, the common one). // GAP: trigger — "leaves
//!   the battlefield" covers more than dies (exile/hand/library zone changes);
//!   only the dies case is modeled.
//! * The targeted effect (a player) + put +1/+1 on EACH creature that player
//!   controls is fully expressible via a per-id `Sequence`.
//! * Evoke is not in the usable keyword surface. // GAP: keyword — Evoke
//!   (alternative cost + sacrifice-on-ETB) not modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Meadowboon");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: keyword — Evoke not in usable keyword surface.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: trigger — "leaves the battlefield" approximated by SelfDies.
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: counters_on_target_players_creatures,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_player()],
        }),
    )
}

fn counters_on_target_players_creatures(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        *p,
    );
    let effects = ids
        .into_iter()
        .map(|id| Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
