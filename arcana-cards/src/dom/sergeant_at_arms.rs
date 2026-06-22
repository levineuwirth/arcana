//! Sergeant-at-Arms — `{2}{W}` 2/3 Human Soldier.
//!
//! * Kicker {2}{W}.
//! * When this creature enters, if it was kicked, create two 1/1 white
//!   Soldier creature tokens.
//!
//! Kicker is not in the usable KeywordAbility surface (keyword GAP'd) and
//! there is no "was kicked" intervening-if condition, so the kicker-
//! conditional ETB is GAP'd: firing the token creation unconditionally would
//! be a materially wrong card.

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
    let name = reg.interner_mut().intern("Sergeant-at-Arms");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: Kicker {2}{W} — not in the usable KeywordAbility surface.
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: "if it was kicked" — no "was kicked" intervening-if.
                intervening_if: None,
                effect: etb_if_kicked,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_if_kicked(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if it was kicked, create two 1/1 white Soldier tokens" — kicker
    //       gate has no condition; firing unconditionally would be wrong.
    Vec::new()
}
