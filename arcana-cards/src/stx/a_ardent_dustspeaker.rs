//! A-Ardent Dustspeaker — `{2}{R}` 3/2 red Minotaur Shaman. "Whenever
//! Ardent Dustspeaker attacks, you may put an instant or sorcery card
//! from your graveyard on the bottom of your library. If you do, exile
//! the top two cards of your library. You may play those cards this
//! turn."
//!
//! We model the attack trigger and the impulse-exile payoff (exile the
//! top two cards; you may play them this turn) via `Effect::ImpulseExile`.
//! The optional "put an instant or sorcery from your graveyard on the
//! bottom of your library" prerequisite cost (and its "if you do" gate)
//! is not expressible as an optional in-effect graveyard-to-library
//! payment, so it is left as a GAP — the impulse payoff fires
//! unconditionally as a best-effort.

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
    let name = reg.interner_mut().intern("A-Ardent Dustspeaker");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: impulse_top_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn impulse_top_two(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the optional "put an instant or sorcery card from your
    // graveyard on the bottom of your library" prerequisite (and the
    // "if you do" gate) is not expressible as an optional cost; the
    // impulse-exile payoff is emitted unconditionally as a best effort.
    vec![Effect::ImpulseExile {
        player: trig.controller,
        count: 2,
    }]
}
