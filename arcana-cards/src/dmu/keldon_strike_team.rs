//! Keldon Strike Team — `{2}{R}` 3/1 Human Warrior.
//!
//! Oracle:
//! * Kicker {1}{W}.
//! * When this creature enters, if it was kicked, create two 1/1 white Soldier
//!   creature tokens.
//! * As long as this creature entered this turn, creatures you control have
//!   haste.
//!
//! Kicker is not in the usable keyword surface (and kicked-status is not
//! tracked), so `keywords` is empty and the kicker is GAP'd. The ETB trigger
//! frame is emitted but its effect is GAP'd: the "if it was kicked"
//! intervening-if has no condition helper and kicked status is unavailable.
//! The haste-granting line is a pure continuous STATIC — GAP'd.

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
    let name = reg.interner_mut().intern("Keldon Strike Team");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Kicker {1}{W} not in the usable keyword surface.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: kicked_soldiers,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
    // GAP (static): "As long as this creature entered this turn, creatures you
    // control have haste." — a pure conditional continuous static.
}

fn kicked_soldiers(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if it was kicked" — no kicked-status accessor / intervening-if
    // helper, so the conditional token creation cannot be expressed.
    Vec::new()
}
