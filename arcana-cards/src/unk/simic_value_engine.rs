//! Simic, Value Engine — `{1}{U}{G}` 3/2 Legendary Creature — Vedalken Octopus Raccoon.
//! "At the beginning of your end step, if more lands entered the battlefield under your control this turn than an opponent had enter during their last turn, draw a card. Then, not counting that draw, if you drew more cards this turn than an opponent drew on their last turn, you may put a land card from your hand onto the battlefield."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Simic, Value Engine");
    let vedalken_sub = reg.interner_mut().intern("Vedalken");
    let octopus_sub = reg.interner_mut().intern("Octopus");
    let raccoon_sub = reg.interner_mut().intern("Raccoon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken_sub);
    subtypes.0.insert(octopus_sub);
    subtypes.0.insert(raccoon_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins { step: Step::End, whose: ControllerConstraint::You },
                intervening_if: None,
                effect: simic_value_engine_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn simic_value_engine_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
