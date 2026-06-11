//! Elfhame Sanctuary — `{1}{G}` enchantment (Invasion, 2000).
//! "At the beginning of your upkeep, you may search your library for a
//! basic land card, reveal that card, put it into your hand, then shuffle.
//! If you do, you skip your draw step this turn."
//!
//! An upkeep trigger resolving a basic-land tutor to hand. GAPs: the "you
//! may" optionality and the "skip your draw step this turn" rider (no
//! skip-step effect).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elfhame Sanctuary");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: fetch_basic,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you may search your library for a basic land card, reveal that card,
/// put it into your hand, then shuffle. If you do, you skip your draw step
/// this turn."
fn fetch_basic(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may" (the tutor resolves unconditionally) and "you skip
    // your draw step this turn" (no skip-step effect) are not expressible.
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC)),
        reveal: true,
    }]
}
