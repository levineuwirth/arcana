//! Kessig Forgemaster // Flameheart Werewolf — `{1}{R}` Human Shaman Werewolf 2/1.
//!
//! Front face (Kessig Forgemaster):
//!   Whenever this creature blocks or becomes blocked by a creature, this creature deals 1 damage
//!   to that creature.
//!   At the beginning of each upkeep, if no spells were cast last turn, transform this creature.
//!
//! Back face (Flameheart Werewolf) — Creature — Werewolf:
//!   Whenever this creature blocks or becomes blocked by a creature, this creature deals 2 damage
//!   to that creature.
//!   At the beginning of each upkeep, if a player cast two or more spells last turn, transform
//!   this creature.
//!
//! # GAPs
//! - "if no spells were cast last turn" and "if a player cast two or more spells last turn"
//!   are werewolf day/night transform conditions that cannot be modeled as an intervening-if
//!   — the upkeep triggers fire unconditionally.
//!   GAP: day/night / spells-cast-last-turn werewolf transform condition not modeled.
//! - The back face BlocksOrBlocked triggered ability (2 damage) is not auto-installed on transform
//!   (abilities live on the CardDefinition, not the face).
//!   GAP: back-face-only triggered ability not modeled (back BlocksOrBlocked deals 2 damage).
//! - The back face upkeep-transform trigger is likewise a front-face approximation only.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
    let name = reg.interner_mut().intern("Kessig Forgemaster");
    let human_sub = reg.interner_mut().intern("Human");
    let shaman_sub = reg.interner_mut().intern("Shaman");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(shaman_sub);
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

    let back_name = reg.interner_mut().intern("Flameheart Werewolf");
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
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face: blocks or becomes blocked — deal 1 damage
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBlocksOrBecomesBlocked,
                intervening_if: None,
                effect: blocks_or_blocked_1,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Upkeep: transform (werewolf day/night condition GAP)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
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
            // GAP: back-face-only triggered ability not modeled (blocks/blocked → 2 damage on back face)
    )
}

fn blocks_or_blocked_1(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(other) = trig.other_combatant() else { return Vec::new(); };
    vec![Effect::DealDamage {
        target: DamageTarget::Object(other),
        amount: 1,
        source: trig.source,
    }]
}

fn transform_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}
