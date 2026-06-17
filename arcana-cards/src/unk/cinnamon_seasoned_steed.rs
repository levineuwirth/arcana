//! Cinnamon, Seasoned Steed — `{1}` 1/1 Legendary Artifact Creature — Food
//! Horse (colorless).
//!
//! Oracle:
//! * Other Food creatures you control have horsemanship. — GAP: static
//!   keyword-granting anthem, not a triggered/activated ability.
//! * At the beginning of your combat step, you may put a +1/+1 counter on
//!   target noncreature Food you control. If you do, it becomes a 0/0 Knight
//!   creature in addition to its other types.
//! * {2}, {T}, Sacrifice Cinnamon: You gain 3 life.
//!
//! The combat-step trigger puts the +1/+1 counter on the chosen target. The
//! "it becomes a 0/0 Knight creature in addition to its other types" rider is
//! GAP'd — `AddType` can add the creature type but cannot grant the Knight
//! subtype, and the conditional "if you do" coupling is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cinnamon, Seasoned Steed");
    let food = reg.interner_mut().intern("Food");
    let horse = reg.interner_mut().intern("Horse");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(food);
    subtypes.0.insert(horse);

    let food_filter = script::subtype_filter(reg, "Food")
        .controlled_by(ControllerConstraint::You)
        .without_types(TypeLine::CREATURE.into());

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "Other Food creatures you control have horsemanship" — static.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::BeginCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: counter_on_food,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(food_filter),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}, Sacrifice Cinnamon, Seasoned Steed: You gain 3 life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_three,
            }),
    )
}

fn counter_on_food(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "it becomes a 0/0 Knight creature in addition to its other types"
    // — Knight subtype grant is not expressible.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn gain_three(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife { player: ctx.controller, amount: 3 }]
}
