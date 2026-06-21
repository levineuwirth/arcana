//! Earthshaker Dreadmaw — `{4}{G}{G}` 6/6 Dinosaur with Trample.
//!
//! Oracle:
//! * Trample.
//! * When this creature enters, draw a card for each other Dinosaur you
//!   control.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Earthshaker Dreadmaw");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_draw_per_dinosaur,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_draw_per_dinosaur(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "for each other Dinosaur you control" — count Dinosaurs you
    // control and subtract this creature (the entering source) which is
    // itself a Dinosaur on the battlefield.
    let dinos = script::count_matching(
        state,
        &script::subtype_filter(reg, "Dinosaur").controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let n = dinos.saturating_sub(1);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::DrawCards {
        player: trig.controller,
        count: n,
    }]
}
