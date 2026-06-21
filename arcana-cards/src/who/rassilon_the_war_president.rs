//! Rassilon, the War President — `{3}{U}{B}` 3/4 Legendary Time Lord
//! Noble. "At the beginning of your upkeep, you lose 2 life and exile the
//! top card of your library. You may play that card for as long as it
//! remains exiled." "Each noncreature spell you cast from exile has
//! conspire."

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rassilon, the War President");
    let time_lord = reg.interner_mut().intern("Time Lord");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(time_lord);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "Each noncreature spell you cast from exile has conspire."
    // — a cost/cast-modifier static granting conspire is not expressible.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: upkeep_lose_and_impulse,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn upkeep_lose_and_impulse(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Faithful: lose 2 life and exile the top card with play permission.
    // FIDELITY GAP: ImpulseExile grants play-until-end-of-turn permission,
    // whereas Rassilon grants it "for as long as it remains exiled" (no EOT
    // lapse). The duration of the permission is the only divergence.
    vec![
        Effect::LoseLife { player: trig.controller, amount: 2 },
        Effect::ImpulseExile { player: trig.controller, count: 1 },
    ]
}
