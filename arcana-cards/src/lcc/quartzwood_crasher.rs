//! Quartzwood Crasher — `{2}{R}{R}{G}` 6/6 Dinosaur Beast with Trample.
//! "Whenever one or more creatures you control with trample deal combat
//! damage to a player, create an X/X green Dinosaur Beast creature token
//! with trample, where X is the amount of damage those creatures dealt to
//! that player."
//!
//! The combat-damage-to-a-player trigger is wired via `DamageDealt`
//! (source = a trample creature you control). GAP: the payload mints an
//! X/X token where X is the *aggregate* damage those creatures dealt; a
//! `TokenDefinition` only takes a fixed `PtValue::Fixed`, so a dynamic-P/T
//! token is not expressible. The effect body is therefore GAP'd.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quartzwood Crasher");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
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
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .with_keyword(KeywordAbility::Trample),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: make_token,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_token(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: create an X/X token where X is the aggregate combat damage dealt;
    //      TokenDefinition only supports a fixed PtValue, so a dynamic-P/T
    //      token is not expressible.
    Vec::new()
}
