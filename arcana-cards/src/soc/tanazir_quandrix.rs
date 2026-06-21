//! Tanazir Quandrix — `{3}{G}{U}` 4/4 Legendary Elder Dragon with Flying,
//! Trample.
//!
//! Oracle:
//! * Flying, trample — keywords.
//! * When Tanazir Quandrix enters, double the number of +1/+1 counters on
//!   target creature you control. — ETB targeted trigger. No "double
//!   counters" primitive exists; doubling is modeled by reading the current
//!   +1/+1 count and adding that many more (N → 2N).
//! * Whenever Tanazir Quandrix attacks, you may have the base power and
//!   toughness of other creatures you control become equal to Tanazir
//!   Quandrix's power and toughness until end of turn. — `SelfAttacks`
//!   trigger setting each other creature you control's base P/T to Tanazir's
//!   P/T via per-id `SetBasePT`. (The "you may" optional is applied
//!   unconditionally; the "other" exclusion is approximated since the filter
//!   has no exclude-source field — Tanazir's own base P/T set to its current
//!   P/T is a no-op.)

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tanazir Quandrix");
    let elder = reg.interner_mut().intern("Elder");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_double_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_clone_pt,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_double_counters(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let current = state
        .objects
        .get(*id)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    if current == 0 {
        return Vec::new();
    }
    // Doubling N counters = adding N more.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: current,
    }]
}

fn attack_clone_pt(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let power = script::power_of(state, trig.source);
    let toughness = script::toughness_of(state, trig.source);
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    ids.into_iter()
        .map(|id| Effect::SetBasePT {
            target: id,
            power,
            toughness,
            duration: Duration::EndOfTurn,
        })
        .collect()
}
