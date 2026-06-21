//! A-Death-Priest of Myrkul — `{2}{B}{B}` 3/3 Tiefling Cleric.
//!
//! "Skeletons, Vampires, and Zombies you control get +1/+1." (static anthem)
//! "At the beginning of your end step, if a creature died this turn, create
//! a 1/1 black Skeleton creature token."
//!
//! The end-step token trigger (with an intervening-if for "a creature died
//! this turn") is faithful. The tribal +1/+1 anthem is a pure static
//! continuous ability with no documented Effect representation, so it is
//! GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::conditions;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::TokenDefinition;

// GAP (static): "Skeletons, Vampires, and Zombies you control get +1/+1" —
// a pure static continuous anthem with no documented Effect/static API for
// this card class.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Death-Priest of Myrkul");
    let tiefling = reg.interner_mut().intern("Tiefling");
    let cleric = reg.interner_mut().intern("Cleric");
    let _skeleton = reg.interner_mut().intern("Skeleton");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tiefling);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: Some(if_creature_died),
            effect: make_skeleton,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn if_creature_died(
    s: &GameState,
    _src: ObjectId,
    _you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::a_creature_died_this_turn(s)
}

fn make_skeleton(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let skeleton = reg.interner().lookup("Skeleton").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(skeleton);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: skeleton,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
