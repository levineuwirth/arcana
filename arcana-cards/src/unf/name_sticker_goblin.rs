//! "Name Sticker" Goblin — `{2}{R}` 2/2 Goblin Guest.
//! "When this creature enters from anywhere other than a graveyard or exile,
//!  if it's on the battlefield and you control 9 or fewer creatures named
//!  \"Name Sticker\" Goblin, roll a 20-sided die.
//!  1-6 | Add {R}{R}{R}{R}.  7-14 | Add {R}{R}{R}{R}{R}.  15-20 | Add {R}{R}{R}{R}{R}{R}."

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("\"Name Sticker\" Goblin");
    let goblin = reg.interner_mut().intern("Goblin");
    let guest = reg.interner_mut().intern("Guest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(guest);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: the "enters from anywhere OTHER THAN a graveyard or exile"
            // qualifier is unmodeled (plain SelfEntersBattlefield used).
            // GAP: effect — "roll a 20-sided die" with banded mana outcomes is
            // not expressible (no die-roll primitive); the dynamic mana amount
            // cannot be produced, so the effect body is empty.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: Some(if_nine_or_fewer_named),
                effect: roll_for_mana,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_nine_or_fewer_named(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    let nm = reg.interner().lookup("\"Name Sticker\" Goblin");
    let filter = ObjectFilter { name: nm, ..ObjectFilter::creature() };
    conditions::you_control_at_most(s, you, &filter, 9)
}

fn roll_for_mana(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: 20-sided die roll with banded mana outcomes — no die-roll effect.
    Vec::new()
}
