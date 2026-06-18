//! Rank Officer — `{3}{B}` 3/1 Zombie Soldier.
//! "When this creature enters, you may discard a card. If you do,
//! create a 2/2 black Zombie creature token.
//! {1}{B}, {T}, Exile a creature card from your graveyard: Each
//! opponent loses 2 life."
//!
//! The activated ability is expressed with its mana + tap costs and
//! the "each opponent loses 2 life" effect; the additional "Exile a
//! creature card from your graveyard" cost has no `ActivationCost`
//! field and is GAP'd. The ETB "you may discard a card. If you do, …"
//! trigger is
//! GAP'd — `OptionalPayment` only supports Mana/Life costs, so the
//! "may discard, if you do create" gate is not expressible without
//! making the token creation unconditional (materially wrong).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rank Officer");
    let zombie = reg.interner_mut().intern("Zombie");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: ETB "you may discard a card. If you do, create a 2/2
            // black Zombie creature token." — OptionalPayment supports
            // only Mana/Life costs, so the may-discard / if-you-do gate
            // is not expressible.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}, {T}, Exile a creature card from your graveyard: Each opponent loses 2 life.".into(),
                // GAP: the "Exile a creature card from your graveyard"
                // portion of the cost has no ActivationCost field; only
                // the mana + tap costs are modeled.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: drain_each_opponent,
            }),
    )
}

fn etb_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may discard a card. If you do, create a 2/2 black
    // Zombie creature token." — not expressible (see register comment).
    Vec::new()
}

fn drain_each_opponent(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opps = script::opponents(state, ctx.controller);
    opps.into_iter()
        .map(|p| Effect::LoseLife { player: p, amount: 2 })
        .collect()
}
