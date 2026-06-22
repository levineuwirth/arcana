//! Szarel, Genesis Shepherd — `{2}{B}{R}{G}` Legendary 2/5 Insect Druid.
//! Flying.
//! You may play lands from your graveyard.
//! Whenever you sacrifice another nontoken permanent during your turn,
//! put a number of +1/+1 counters equal to Szarel's power on up to one
//! other target creature.
//!
//! GAP: "You may play lands from your graveyard" is a static play-zone
//! permission with no expressible Effect/keyword.
//! GAP (intervening-if): "during your turn" — no conditions:: predicate
//! exposes whose turn it is for a sacrifice trigger, so the counter
//! trigger fires on any turn (a documented over-fire). The sacrifice
//! trigger, dynamic counter count (= Szarel's power), and up-to-one
//! target creature are all wired.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Szarel, Genesis Shepherd");
    let insect = reg.interner_mut().intern("Insect");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::Sacrificed {
                    filter: ObjectFilter::permanent()
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(),
                },
                intervening_if: None,
                effect: sacrifice_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn sacrifice_counters(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let n = script::power_of(state, trig.source).max(0) as u32;
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: n,
    }]
}
