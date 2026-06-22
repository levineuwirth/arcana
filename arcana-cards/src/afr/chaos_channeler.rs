//! Chaos Channeler — `{2}{R}{R}` 4/3 Creature — Human Shaman.
//! "Wild Magic Surge — Whenever this creature attacks, roll a d20.
//!  1—9  | Exile the top card of your library. You may play it this turn.
//!  10—19| Exile the top two cards of your library. You may play them this turn.
//!  20   | Exile the top three cards of your library. You may play them this turn."
//!
//! The attack trigger is wired as an impulse exile of the top card
//! with play-permission. There is no die-roll Effect, so the d20
//! band scaling (1 / 2 / 3 cards) is GAP'd — we emit the minimum band
//! (1 card) as the deterministic best-effort.

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
    let name = reg.interner_mut().intern("Chaos Channeler");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: wild_magic_surge,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn wild_magic_surge(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "roll a d20" band scaling (1 / 2 / 3 cards) — no die-roll
    // Effect exists; emit the minimum band (exile top card, may play it).
    vec![Effect::ImpulseExile {
        player: trig.controller,
        count: 1,
    }]
}
