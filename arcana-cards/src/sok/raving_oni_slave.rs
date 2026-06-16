//! Raving Oni-Slave — `{1}{B}` 3/3 black Ogre Warrior creature.
//! "When this creature enters or leaves the battlefield, you lose 3 life if you don't
//! control a Demon."
//!
//! # Notes
//! Two triggers: ETB and leaves-battlefield. For leaves-battlefield, using SelfDies as
//! closest approximation (no general "leaves battlefield" TriggerCondition).
//! GAP: "when leaves the battlefield" in general — only SelfDies available; other LTB
//! events (exile, bounce, transform) not covered.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Raving Oni-Slave");
    let ogre = reg.interner_mut().intern("Ogre");
    let warrior = reg.interner_mut().intern("Warrior");
    let _demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: lose_life_if_no_demon,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // GAP: "leaves the battlefield" in general — using SelfDies as approximation.
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: lose_life_if_no_demon,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn lose_life_if_no_demon(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let demon_filter = script::subtype_filter(reg, "Demon")
        .controlled_by(ControllerConstraint::You);
    if script::count_matching(state, &demon_filter, trig.controller) == 0 {
        vec![Effect::LoseLife { player: trig.controller, amount: 3 }]
    } else {
        Vec::new()
    }
}
