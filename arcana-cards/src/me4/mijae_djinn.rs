//! Mijae Djinn — `{R}{R}{R}` 6/3 red Djinn.
//! "Whenever this creature attacks, flip a coin. If you lose the flip, remove
//! this creature from combat and tap it."
//!
//! GAP: "remove this creature from combat" — no Effect variant exists for
//! removing a permanent from combat. The lose branch emits only Tap (the tap
//! half is expressible); the "remove from combat" portion is omitted.

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
    let name = reg.interner_mut().intern("Mijae Djinn");
    let djinn = reg.interner_mut().intern("Djinn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_flip_coin,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attacks_flip_coin(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Win the flip: nothing happens. Lose the flip: tap this creature.
    // GAP: "remove this creature from combat" has no Effect variant; only the
    // tap half is modeled.
    vec![Effect::FlipCoin {
        player: trig.controller,
        win: Box::new(Effect::Sequence(vec![])),
        lose: Some(Box::new(Effect::Tap { target: trig.source })),
    }]
}
