//! Scourge of Numai — `{3}{B}` 4/4 black Demon Spirit creature.
//! "At the beginning of your upkeep, you lose 2 life if you don't control an Ogre."
//!
//! # Notes
//! GAP: intervening-if "if you don't control an Ogre" — not expressible as intervening_if.
//! Modeled as upkeep trigger; GAP noted.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scourge of Numai");
    let demon = reg.interner_mut().intern("Demon");
    let spirit = reg.interner_mut().intern("Spirit");
    let _ogre = reg.interner_mut().intern("Ogre");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
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
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                // GAP: intervening_if — "if you don't control an Ogre" not expressible.
                intervening_if: None,
                effect: upkeep_lose_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_lose_life(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let ogre_filter = script::subtype_filter(reg, "Ogre")
        .controlled_by(ControllerConstraint::You);
    if script::count_matching(state, &ogre_filter, trig.controller) == 0 {
        vec![Effect::LoseLife { player: trig.controller, amount: 2 }]
    } else {
        Vec::new()
    }
}
