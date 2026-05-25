//! Kynaios and Tiro of Meletis — `{R}{G}{W}{U}` 2/8 red-green-white-blue Legendary
//! Human Soldier.
//! "At the beginning of your end step, draw a card. Each player may put a land card
//! from their hand onto the battlefield, then each opponent who didn't draws a card."
//! GAP: "each player may put a land from hand onto battlefield" is not in the engine
//! effect catalog; drawing a card for controller and each opponent (who didn't land) is
//! approximated as DrawCards for all opponents.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kynaios and Tiro of Meletis");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_group_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_group_draw(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each player may put a land from hand onto battlefield" not in engine catalog.
    // Best-effort: controller draws, then each opponent draws (approximating "didn't put land").
    let mut effects = vec![Effect::DrawCards { player: trig.controller, count: 1 }];
    for opp in script::opponents(state, trig.controller) {
        effects.push(Effect::DrawCards { player: opp, count: 1 });
    }
    vec![Effect::Sequence(effects)]
}
