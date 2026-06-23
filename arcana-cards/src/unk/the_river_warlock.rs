//! The River Warlock — `{1}{B}{R}{G}` 1/1 Legendary Human Warlock.
//!
//! Oracle:
//! * During your turn, The River Warlock has flying, lifelink, and
//!   indestructible.  (Conditional continuous static — GAP.)
//! * Whenever The River Warlock attacks, you and defending player each
//!   draw a card, then discard a card. If you discarded a card with
//!   equal or greater mana value than the card that player discarded,
//!   look at their hand. Exile a nonland card from it until The River
//!   Warlock leaves the battlefield.
//!
//! Implemented: the attack trigger's "you and defending player each draw
//! a card, then discard a card" — both players draw one, then each
//! discards one. The conditional clause (compare discarded mana values,
//! look at the defender's hand, exile-until-leaves) is GAP'd: it depends
//! on the mana values of the just-discarded cards, which there is no
//! primitive to capture and compare at resolution time.
//!
//! The "During your turn, has flying/lifelink/indestructible" line is a
//! turn-gated continuous static and is GAP'd.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The River Warlock");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);

    // GAP: static — "During your turn, The River Warlock has flying,
    // lifelink, and indestructible": turn-gated continuous self-static.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_draw_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_draw_discard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
    ];
    if let Some(defender) = trig.defending_player() {
        effects.push(Effect::DrawCards { player: defender, count: 1 });
    }
    // Then each player discards a card.
    effects.push(Effect::Discard {
        player: trig.controller,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    });
    if let Some(defender) = trig.defending_player() {
        effects.push(Effect::Discard {
            player: defender,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        });
    }
    // GAP: "If you discarded a card with equal or greater mana value than
    // the card that player discarded, look at their hand and exile a
    // nonland card from it until The River Warlock leaves the battlefield."
    // Comparing the mana values of the two just-discarded cards is not
    // expressible at resolution time.
    effects
}
