//! Thunderous Snapper — `{G/U}{G/U}{G/U}{G/U}` 4/4 green-blue Turtle Hydra.
//! "Whenever you cast a spell with mana value 5 or greater, draw a
//! card."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thunderous Snapper");
    let turtle = reg.interner_mut().intern("Turtle");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(turtle);
    subtypes.0.insert(hydra);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G/U}{G/U}{G/U}{G/U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_min_cmc(5)),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: high_cmc_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn high_cmc_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
