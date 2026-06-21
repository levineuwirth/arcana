//! Tomb of Horrors Adventurer — `{5}{U}` 4/4 Elf Monk.
//! * "When this creature enters, you take the initiative." — the
//!   initiative mechanic is not modeled (no Effect::TakeInitiative); the
//!   ETB trigger is wired with a GAP'd effect.
//! * "Whenever you cast your second spell each turn, copy it. If you've
//!   completed a dungeon, copy that spell twice instead. You may choose
//!   new targets for the copies." — the trigger fires on the controller's
//!   second spell of the turn (intervening-if), but the copy effect is
//!   GAP'd: a SpellCast trigger exposes no accessor for the triggering
//!   spell's stack object id, and "completed a dungeon" is not a
//!   queryable condition.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tomb of Horrors Adventurer");
    let elf = reg.interner_mut().intern("Elf");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(monk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: take_initiative,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: Some(if_second_spell),
                effect: copy_that_spell,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_second_spell(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::spells_cast_this_turn(s, &ObjectFilter::default(), you) == 2
}

fn take_initiative(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you take the initiative." — the initiative mechanic is not
    // modeled (no Effect::TakeInitiative).
    Vec::new()
}

fn copy_that_spell(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "copy it" (and "copy that spell twice if you've completed a
    // dungeon") — no accessor for the triggering spell's stack object id
    // from a SpellCast trigger, and "completed a dungeon" is not queryable.
    Vec::new()
}
