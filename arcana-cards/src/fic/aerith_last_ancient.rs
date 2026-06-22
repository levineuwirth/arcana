//! Aerith, Last Ancient — `{2}{G}{W}` 3/5 Legendary Human Cleric Druid
//! with Lifelink.
//! "Raise — At the beginning of your end step, if you gained life this
//! turn, return target creature card from your graveyard to your hand.
//! If you gained 7 or more life this turn, return that card to the
//! battlefield instead."
//!
//! The intervening-if ("if you gained life this turn") gates the
//! trigger. The "7 or more life → battlefield instead" upgrade is a
//! resolution-time amount the available script helpers can't read
//! (no "life gained this turn" amount accessor — only the boolean
//! you_gained_life_this_turn predicate), so the resolver always
//! returns the card to hand and the battlefield upgrade is GAP'd.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::effects::KeywordAbility;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aerith, Last Ancient");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
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
            intervening_if: Some(if_gained_life),
            effect: raise_return,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature(),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn if_gained_life(s: &GameState, _src: ObjectId, you: arcana_core::types::PlayerId, _reg: &CardRegistry) -> bool {
    conditions::you_gained_life_this_turn(s, you)
}

fn raise_return(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: the "if you gained 7 or more life this turn, return that card to
    // the battlefield instead" upgrade needs a resolution-time amount of
    // life gained this turn; only a boolean you_gained_life_this_turn
    // predicate exists, so the card is always returned to hand.
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}
