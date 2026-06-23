//! Bringer of the Blue Dawn — `{7}{U}{U}` 5/5 Bringer with Trample.
//! "You may pay {W}{U}{B}{R}{G} rather than pay this spell's mana cost."
//! "At the beginning of your upkeep, you may draw two cards."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bringer of the Blue Dawn");
    let bringer = reg.interner_mut().intern("Bringer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bringer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "You may pay {W}{U}{B}{R}{G} rather than pay this spell's mana
    //      cost." — an alternative casting cost (static casting-time
    //      permission); no triggered / activated primitive expresses an
    //      alternative cast cost.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_draw_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_draw_two(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you may draw two cards" — drawing is pure upside; modeled as drawing
    // two (no optional-draw primitive is demonstrated).
    vec![Effect::DrawCards { player: trig.controller, count: 2 }]
}
