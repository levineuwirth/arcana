//! The Cobra King — `{4}{G}{U}` 5/2 Legendary Creature — Snake Hawk Warrior.
//! At the beginning of each player's upkeep, create a 1/1 blue Serpent creature
//! token named Cobra Coil. When you do, if you control five or more Snakes
//! and/or Serpents, choose one —
//! • Strike first — Target Snake or Serpent you control fights target creature
//!   an opponent controls.
//! • Strike hard — Put a +1/+1 counter on each Snake and Serpent you control.
//!
//! GAP (reflexive modal): the "When you do, if you control five or more Snakes
//! and/or Serpents, choose one —" clause is a reflexive triggered ability with a
//! modal choice. Modal dispatch (with_mode_effects) is a spell-level concept not
//! available inside a triggered ability's resolution, so the fight / counter
//! modes and their intervening-if are not wired. The repeating Serpent-token
//! creation IS expressed.

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
    let name = reg.interner_mut().intern("The Cobra King");
    let snake = reg.interner_mut().intern("Snake");
    let hawk = reg.interner_mut().intern("Hawk");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(hawk);
    subtypes.0.insert(warrior);
    // Token name + subtype interned for the resolver.
    let _serpent = reg.interner_mut().intern("Serpent");
    let _coil = reg.interner_mut().intern("Cobra Coil");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::Any,
            },
            intervening_if: None,
            effect: create_cobra_coil,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn create_cobra_coil(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let coil = match reg.interner().lookup("Cobra Coil") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let serpent = match reg.interner().lookup("Serpent") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: arcana_core::effects::TokenDefinition {
            name: coil,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
