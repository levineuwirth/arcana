//! Gavony Dawnguard — `{1}{W}{W}` 3/3 Human Soldier.
//!
//! "Ward {1}
//!  If it's neither day nor night, it becomes day as this creature
//!  enters.
//!  Whenever day becomes night or night becomes day, look at the top
//!  four cards of your library. You may reveal a creature card with mana
//!  value 3 or less from among them and put it into your hand. Put the
//!  rest on the bottom of your library in any order."
//!
//! Decomposition: Ward {1} keyword + an ETB trigger that turns it day
//! (gated by an intervening-if checking it's currently neither day nor
//! night). The "whenever day becomes night or night becomes day" trigger
//! has no engine TriggerCondition (no day/night-transition event), so it
//! is GAP'd.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::DayNight;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gavony Dawnguard");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{1}").expect("valid cost"))],
        ..Default::default()
    };
    // GAP: "Whenever day becomes night or night becomes day, look at the
    // top four cards…" — there is no day/night-transition TriggerCondition.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: Some(if_neither_day_nor_night),
                effect: become_day,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_neither_day_nor_night(
    s: &GameState,
    _src: ObjectId,
    _you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    !conditions::it_is_day(s) && !conditions::it_is_night(s)
}

fn become_day(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::SetDayNight { value: DayNight::Day }]
}
