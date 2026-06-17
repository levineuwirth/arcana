//! Michelangelo, the Heart — `{1}{G}` 2/1 Legendary Mutant Ninja
//! Turtle with Trample. "Raid — At the beginning of your second main
//! phase, if you attacked this turn, put a +1/+1 counter on target
//! creature and create a Food token. Partner — Character select."
//!
//! Trample is a base keyword. The Raid trigger fires at the beginning
//! of the post-combat (second) main phase, gated by an intervening-if
//! ("if you attacked this turn"); it puts a +1/+1 counter on a target
//! creature and creates a Food token.
//!
//! GAP: Food (the token-granting keyword) and Partner are not
//! `KeywordAbility` variants — they are unmodeled, so omitted from the
//! keyword line. (The Food *token* itself is created by the trigger
//! via `CreateCommodityToken`.)

use arcana_core::conditions;
use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Michelangelo, the Heart");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(turtle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::PostCombatMain,
                whose: ControllerConstraint::You,
            },
            intervening_if: Some(if_you_attacked_this_turn),
            effect: raid_counter_and_food,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_creature()],
        }),
    )
}

fn if_you_attacked_this_turn(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::you_attacked_this_turn(s, you)
}

fn raid_counter_and_food(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Sequence(vec![
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Food,
            count: 1,
        },
    ])]
}
