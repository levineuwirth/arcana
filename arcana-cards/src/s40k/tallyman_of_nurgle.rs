//! Tallyman of Nurgle — `{2}{B}` 2/3 Astartes Warrior.
//! Lifelink.
//! The Seven-fold Chant — At the beginning of your end step, if a creature
//! died this turn, you draw a card and lose 1 life. If seven or more creatures
//! died this turn, instead you draw seven cards and lose 7 life.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tallyman of Nurgle");
    let astartes = reg.interner_mut().intern("Astartes");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(astartes);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: Some(if_a_creature_died),
            effect: seven_fold_chant,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn if_a_creature_died(s: &GameState, _src: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::a_creature_died_this_turn(s)
}

fn seven_fold_chant(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let died = script::creatures_died_this_turn(state);
    let (draw, life) = if died >= 7 { (7, 7) } else { (1, 1) };
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: draw,
        },
        Effect::LoseLife {
            player: trig.controller,
            amount: life,
        },
    ]
}
