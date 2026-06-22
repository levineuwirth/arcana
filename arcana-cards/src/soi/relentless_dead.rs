//! Relentless Dead — `{B}{B}` 2/2 Creature — Zombie.
//!
//! Oracle:
//! * Menace — keyword.
//! * "When this creature dies, you may pay {B}. If you do, return it to
//!   its owner's hand." — SelfDies trigger; OptionalPayment of {B}, then
//!   return the dying object from the graveyard to hand.
//! * "When this creature dies, you may pay {X}. If you do, return another
//!   target Zombie creature card with mana value X from your graveyard to
//!   the battlefield." — SelfDies trigger fires, but GAP'd: a variable
//!   {X} payment whose value X constrains the returned card's mana value
//!   is not expressible (OptionalPayment takes a fixed mana cost).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
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
    let name = reg.interner_mut().intern("Relentless Dead");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: pay_b_return_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: pay_x_reanimate_zombie,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pay_b_return_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let id = trig.dying_object().unwrap_or(trig.source);
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{B}").expect("valid cost")),
        then: Box::new(Effect::ReturnFromGraveyardToHand { target: id }),
        else_effect: None,
    }]
}

fn pay_x_reanimate_zombie(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "pay {X}, return a Zombie creature card with mana value X" — a
    // variable {X} payment that constrains the target's mana value is not
    // expressible (OptionalPayment takes a fixed cost).
    Vec::new()
}
