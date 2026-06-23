//! Guru Pathik — `{2}{G/U}{G/U}` 2/4 Legendary Human Monk Ally (green/blue).
//!
//! * "When Guru Pathik enters, look at the top five cards of your library. You
//!   may reveal a Lesson, Saga, or Shrine card from among them and put it into
//!   your hand. Put the rest on the bottom of your library in a random order."
//!   — `SelfEntersBattlefield` → `Effect::DigTopN` with a subtype-OR filter and
//!   `DigRest::BottomRandom`.
//! * "Whenever you cast a Lesson, Saga, or Shrine spell, put a +1/+1 counter on
//!   another target creature you control." — `SpellCast` (caster You, filtered
//!   to the three subtypes) targeting another creature you control, then
//!   `Effect::AddCounters`.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Guru Pathik");
    let human = reg.interner_mut().intern("Human");
    let monk = reg.interner_mut().intern("Monk");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(monk);
    subtypes.0.insert(ally);

    // Subtype-OR filter for the cast trigger: Lesson, Saga, or Shrine.
    let lesson = reg.interner_mut().intern("Lesson");
    let saga = reg.interner_mut().intern("Saga");
    let shrine = reg.interner_mut().intern("Shrine");
    let spell_filter = ObjectFilter::new().with_subtypes_any(vec![lesson, saga, shrine]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G/U}{G/U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_dig,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(spell_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: counter_on_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_dig(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Build the subtype-OR filter from the read-only interner.
    let syms: Vec<_> = ["Lesson", "Saga", "Shrine"]
        .into_iter()
        .filter_map(|s| reg.interner().lookup(s))
        .collect();
    let filter = ObjectFilter::new().with_subtypes_any(syms);
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 5,
        filter: Some(filter),
        rest: DigRest::BottomRandom,
    }]
}

fn counter_on_target(
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
