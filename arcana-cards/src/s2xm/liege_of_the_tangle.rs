//! Liege of the Tangle — `{6}{G}{G}` 8/8 Elemental with Trample.
//! "Whenever this creature deals combat damage to a player, you may choose any number
//! of target lands you control and put an awakening counter on each of them." We
//! target any number of lands you control and add an awakening counter to each.
//! "Each of those lands is an 8/8 green Elemental … as long as it has an awakening
//! counter on it. They're still lands." — continuous animation keyed on the counter,
//! GAP (no expressible counter-gated continuous effect).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liege of the Tangle");
    let elemental = reg.interner_mut().intern("Elemental");
    let _awakening = reg.interner_mut().intern("awakening");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: awaken_lands,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Any,
                controller: None,
            }],
        }),
    )
}

fn awaken_lands(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let kind = match reg.interner().lookup("awakening") {
        Some(sym) => CounterKind::Named(sym),
        None => return Vec::new(),
    };
    let effs: Vec<Effect> = trig
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::AddCounters { target: *id, kind, count: 1 }),
            _ => None,
        })
        .collect();
    // GAP: "each of those lands is an 8/8 green Elemental as long as it has an
    // awakening counter" — counter-gated continuous animation not expressible.
    effs
}
