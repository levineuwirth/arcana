//! Auntie Blyte, Bad Influence — `{2}{R}` 2/2 Legendary Devil Advisor.
//! Flying.
//! Whenever a source you control deals damage to you, put that many
//! +1/+1 counters on Auntie Blyte.
//! {1}{R}, {T}, Remove X +1/+1 counters: deals X damage to any target. (GAP — see below)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Auntie Blyte, Bad Influence");
    let devil = reg.interner_mut().intern("Devil");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(devil);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::permanent()
                        .controlled_by(arcana_core::targets::ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                intervening_if: None,
                effect: add_counters_for_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
    // GAP: "{1}{R}, {T}, Remove X +1/+1 counters from ~: deals X damage" — a
    // variable-X counter-removal cost paired with X-scaled damage is not
    // expressible (remove_self_counter is a fixed count, no X-removal cost field).
}

fn add_counters_for_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: n,
    }]
}
