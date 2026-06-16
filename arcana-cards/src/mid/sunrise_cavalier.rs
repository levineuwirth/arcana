//! Sunrise Cavalier — `{1}{R}{W}` 3/3 Human Knight with Trample and Haste.
//! "If it's neither day nor night, it becomes day as this creature enters.
//!  Whenever day becomes night or night becomes day, put a +1/+1 counter on
//!  target creature you control."
//!
//! Trample and Haste are base keywords.
//! The "becomes day as this creature enters" half is modeled as a
//! SelfEntersBattlefield trigger emitting Effect::SetDayNight(Day). The
//! "if it's neither day nor night" intervening-if gate is GAP'd (no
//! documented day/night condition predicate), so the trigger fires
//! unconditionally as a best-effort.
//! The "Whenever day becomes night or night becomes day" trigger is GAP'd —
//! there is no day/night-transition TriggerCondition variant.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::DayNight;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sunrise Cavalier");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: trigger — "Whenever day becomes night or night becomes day, put a
    // +1/+1 counter on target creature you control": no day/night-transition
    // TriggerCondition variant exists, so that ability is omitted.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: intervening-if — "if it's neither day nor night" has no
                // documented condition predicate; trigger fires unconditionally.
                intervening_if: None,
                effect: etb_become_day,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_become_day(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::SetDayNight { value: DayNight::Day }]
}
