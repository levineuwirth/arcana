//! Impressive Rat — `{2}{B}` 3/2 black Rat. "When Impressive Rat dies,
//! put a hole counter on target land you don't control. Whenever that
//! land becomes tapped, you investigate."
//!
//! GAP: the Investigate keyword has no usable KeywordAbility variant, and
//! the "whenever that land becomes tapped, you investigate" rider is a
//! granted triggered ability attached to the targeted land — not
//! expressible with the catalog. Only the dying trigger that puts a hole
//! counter on a target land you don't control is modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
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
    let name = reg.interner_mut().intern("Impressive Rat");
    let rat = reg.interner_mut().intern("Rat");
    let _hole = reg.interner_mut().intern("hole");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Investigate keyword not in usable KeywordAbility surface.
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_hole_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::LAND.into())
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn dies_hole_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let hole = reg.interner().lookup("hole").expect("hole interned during register()");
    // GAP: "Whenever that land becomes tapped, you investigate" — granting a
    // triggered ability to the counter-marked land isn't expressible; only
    // the hole-counter placement is modeled here.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::Named(hole),
        count: 1,
    }]
}
