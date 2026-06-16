//! Doctor Octopus, Master Planner — `{5}{U}{B}` 4/8 Legendary Human Scientist
//! Villain.
//! Other Villains you control get +2/+2 (static anthem — GAP).
//! Your maximum hand size is eight (static — GAP).
//! At the beginning of your end step, if you have fewer than eight cards in
//!   hand, draw cards equal to the difference.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Doctor Octopus, Master Planner");
    let human = reg.interner_mut().intern("Human");
    let scientist = reg.interner_mut().intern("Scientist");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scientist);
    subtypes.0.insert(villain);

    // GAP (static): "Other Villains you control get +2/+2" — anthem static not
    //   expressible in this card class.
    // GAP (static): "Your maximum hand size is eight" — max-hand-size static not
    //   expressible.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: end_step_draw_up,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn end_step_draw_up(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // The "if you have fewer than eight cards" gate is folded into the dynamic
    // draw count: 8 - hand_size, clamped at 0 (drawing 0 when hand >= 8 is
    // behaviorally identical to not firing).
    let hand = script::hand_size(state, trig.controller);
    let count = 8u32.saturating_sub(hand);
    if count == 0 {
        return Vec::new();
    }
    vec![Effect::DrawCards {
        player: trig.controller,
        count,
    }]
}
