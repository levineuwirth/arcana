//! Quilled Greatwurm — `{4}{G}{G}` 7/7 green Wurm with Trample.
//!
//! * Trample — keyword line.
//! * "Whenever a creature you control deals combat damage during your
//!   turn, put that many +1/+1 counters on it." → a `DamageDealt`
//!   trigger (combat-only) whose source is a creature you control.
//!   GAP (effect): there is no accessor for the damage-SOURCE object at
//!   resolution (no `trig.damage_source()`), so "put that many counters
//!   on IT" cannot identify the damaging creature; and the trigger
//!   itself can't be restricted to "during your turn". The trigger is
//!   wired; its effect is GAP'd.
//! * GAP: "You may cast this card from your graveyard by removing six
//!   counters from among creatures you control in addition to paying its
//!   other costs." — an alternate-cost cast-from-graveyard permission;
//!   not a triggered/activated ability and not expressible here.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quilled Greatwurm");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::AnyTarget,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn combat_damage_counters(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put that many +1/+1 counters on it" — no damage-source
    // object accessor to identify the damaging creature ("it"), and the
    // trigger can't be restricted to "during your turn".
    Vec::new()
}
