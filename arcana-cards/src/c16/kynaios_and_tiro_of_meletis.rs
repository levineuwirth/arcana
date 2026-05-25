//! Kynaios and Tiro of Meletis — `{R}{G}{W}{U}` 2/8 red/green/white/blue Legendary Human Soldier.
//! "At the beginning of your end step, draw a card. Each player may put a land card from
//! their hand onto the battlefield, then each opponent who didn't draws a card."
//! GAP: effect — "each player may put a land from hand onto battlefield" (optional land drop)
//! not in Effect catalog; emitting DrawCards for self only as partial.

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
        keywords: vec![],
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
                effect: on_end_step,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_end_step(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: effect — "each player may put a land from hand onto battlefield" not in catalog;
    // draw card for self + draw card for each opponent (the "didn't put land" fallback) as best effort
    let mut effects = vec![Effect::DrawCards { player: trig.controller, count: 1 }];
    for opp in script::opponents(state, trig.controller) {
        effects.push(Effect::DrawCards { player: opp, count: 1 });
    }
    effects
}
