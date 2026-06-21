//! A-Ocelot Pride — `{1}{W}` 1/1 Cat with First strike and Lifelink.
//!
//! Oracle:
//! * First strike, lifelink.
//! * Ascend.
//! * At the beginning of your end step, if you gained life this turn, create a
//!   1/1 white Cat creature token. Then if you have the city's blessing, for
//!   each token you control that entered this turn, create a token that's a copy
//!   of it.
//!
//! GAP: Ascend is not in the usable keyword surface — omitted.
//! GAP: "Then if you have the city's blessing, for each token … create a copy"
//! — there is no city's-blessing predicate and no "tokens that entered this
//! turn" set; that clause is dropped. The "create a 1/1 white Cat" body and its
//! "gained life this turn" intervening-if are wired.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Ocelot Pride");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_gained_life_this_turn),
                effect: make_cat_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_gained_life_this_turn(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::you_gained_life_this_turn(s, you)
}

fn make_cat_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let cat = reg.interner().lookup("Cat").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: cat,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
    // GAP: "Then if you have the city's blessing, for each token you control
    // that entered this turn, create a token that's a copy of it" — no
    // city's-blessing predicate / entered-this-turn token set.
}
