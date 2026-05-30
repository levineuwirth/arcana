//! Hinterland Hermit // Hinterland Scourge — `{1}{R}` Human Werewolf 2/1.
//!
//! Front face (Hinterland Hermit):
//!   At the beginning of each upkeep, if no spells were cast last turn, transform this creature.
//!
//! Back face (Hinterland Scourge) — Creature — Werewolf:
//!   This creature must be blocked if able.
//!   At the beginning of each upkeep, if a player cast two or more spells last turn, transform
//!   this creature.
//!
//! # GAPs
//! - "If no spells were cast last turn" and "if a player cast two or more spells last turn"
//!   are werewolf day/night transform conditions not expressible as an intervening-if.
//!   The upkeep trigger fires unconditionally.
//!   GAP: day/night / spells-cast-last-turn werewolf transform condition not modeled.
//! - "Must be blocked if able" on the back face is a static constraint not modeled.
//!   GAP: back-face-only "must be blocked if able" static constraint not modeled.
//! - The back face upkeep trigger (transform back to front) fires unconditionally on front face
//!   only. GAP: back-face-only triggered ability not modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hinterland Hermit");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Hinterland Scourge");
    let back_werewolf = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_werewolf);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face upkeep: transform (day/night condition GAP)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: transform_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
            // GAP: back-face-only "must be blocked if able" static constraint not modeled.
            // GAP: back-face-only upkeep trigger (transform back) not modeled.
    )
}

fn transform_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}
