//! Star Charter — `{3}{W}` 3/1 Bat Cleric with Flying.
//!
//! Oracle:
//! * Flying — keyword line.
//! * At the beginning of your end step, if you gained or lost life this
//!   turn, look at the top four cards of your library. You may reveal a
//!   creature card with power 3 or less from among them and put it into
//!   your hand. Put the rest on the bottom of your library in a random
//!   order.
//!
//! The intervening-if ("if you gained or lost life this turn") gates the
//! end-step trigger; the look-at-four is `Effect::DigTopN` with a
//! creature/power<=3 filter and a bottom-random tail.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::effects::DigRest;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Star Charter");
    let bat = reg.interner_mut().intern("Bat");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: Some(if_gained_or_lost_life),
            effect: dig_for_small_creature,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn if_gained_or_lost_life(
    s: &GameState,
    _src: ObjectId,
    you: arcana_core::types::PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::life_gained_this_turn(s, you) > 0 || script::life_lost_this_turn(s, you) > 0
}

fn dig_for_small_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 4,
        filter: Some(ObjectFilter::creature().with_max_power(3)),
        rest: DigRest::BottomRandom,
    }]
}
