//! Mycosynth Wellspring — `{2}` artifact (New Phyrexia).
//! "When this artifact enters or is put into a graveyard from the
//! battlefield, you may search your library for a basic land card, reveal
//! it, put it into your hand, then shuffle."
//!
//! The "enters or is put into a graveyard from the battlefield" double
//! trigger is modeled as TWO triggered abilities (SelfEntersBattlefield +
//! SelfDies) sharing one effect. GAP: the "you may" choice — modeled as
//! mandatory.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mycosynth Wellspring");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: tutor_basic_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: tutor_basic_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn tutor_basic_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may search" — the optional choice is not expressible;
    // modeled as mandatory.
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC)),
        reveal: true,
    }]
}
