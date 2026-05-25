//! Butterbur, Bree Innkeeper — `{2}{G}{W}` 3/3 Legendary Human Peasant.
//! "At the beginning of your end step, if you don't control a Food,
//! create a Food token."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Butterbur, Bree Innkeeper");
    let human = reg.interner_mut().intern("Human");
    let peasant = reg.interner_mut().intern("Peasant");
    let _food = reg.interner_mut().intern("Food");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(peasant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
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

fn on_end_step(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // Intervening-if: "if you don't control a Food" — check via
    // script::count_matching with Food subtype filter.
    let food_filter = script::subtype_filter(reg, "Food")
        .controlled_by(ControllerConstraint::You);
    let food_count = script::count_matching(state, &food_filter, trig.controller);
    if food_count > 0 {
        return Vec::new();
    }
    let food = reg.interner().lookup("Food").expect("Food interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(food);
    let token = TokenDefinition {
        name: food,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: token_subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
