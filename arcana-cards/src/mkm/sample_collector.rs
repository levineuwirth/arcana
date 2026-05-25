//! Sample Collector — `{2}{G}` 2/3 Troll Detective. "Whenever this
//! creature attacks, you may collect evidence 3. When you do, put a
//! +1/+1 counter on target creature you control."
//!
//! GAPs:
//! - Keyword "Collect evidence" is not in the supported keyword
//!   surface — `keywords: vec![]`.
//! - The "you may collect evidence 3" cost (exile cards with total
//!   mana value ≥ 3 from your graveyard) and its reflexive
//!   "When you do" trigger are not modelable; the trigger emits the
//!   +1/+1 counter unconditionally as a best-effort approximation.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
    TargetChoice,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sample Collector");
    let troll = reg.interner_mut().intern("Troll");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(troll);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

/// On attack: put a +1/+1 counter on target creature you control.
///
/// GAP: the oracle gates the counter on "you may collect evidence 3";
/// the engine doesn't model collect-evidence costs or the reflexive
/// "When you do" trigger, so the counter is applied unconditionally.
fn on_attack_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
