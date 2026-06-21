//! All-Star Kicker — `{1}{R}` 2/2 Orc Athlete.
//!
//! * Assist kicker {3}. GAP: Assist/kicker is not an available KeywordAbility
//!   or cost primitive.
//! * When All-Star Kicker enters the battlefield, your team may pass the ball.
//!   Then if All-Star Kicker was kicked, creatures your team controls get +1/+1
//!   and gain haste until end of turn.
//!
//! The ETB trigger is wired but GAP'd: "pass the ball", the "was kicked" gate,
//! and the multiplayer "your team controls" scope are not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("All-Star Kicker");
    let orc = reg.interner_mut().intern("Orc");
    let athlete = reg.interner_mut().intern("Athlete");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(athlete);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_team,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_team(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "pass the ball", the "was kicked" gate, and the multiplayer
    // "creatures your team controls" scope are not expressible.
    Vec::new()
}
