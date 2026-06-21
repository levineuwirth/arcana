//! Cemetery Desecrator — `{4}{B}{B}` 4/4 Zombie with Menace.
//!
//! * Menace — base keyword.
//! * "When this creature enters or dies, exile another card from a graveyard.
//!   When you do, choose one — remove X counters from target permanent; or
//!   target creature an opponent controls gets -X/-X until end of turn, where X
//!   is the mana value of the exiled card." — split into an enters trigger and
//!   a dies trigger. The payoff is a REFLEXIVE modal ("when you do, choose
//!   one") whose X is the mana value of the just-exiled card; a reflexive modal
//!   with an exiled-card-scaled X has no expressible primitive (modal is
//!   spell-ability only, and the exiled card's mana value can't be threaded
//!   into a later effect), so the payoff is GAP'd on both triggers.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Cemetery Desecrator");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: exile_then_modal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: exile_then_modal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn exile_then_modal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile another card from a graveyard. When you do, choose one —
    // remove X counters from target permanent / target creature an opponent
    // controls gets -X/-X, where X is the mana value of the exiled card." A
    // reflexive modal scaled by the exiled card's mana value is not expressible
    // (modal is spell-ability only; the exiled card's mv can't be threaded).
    Vec::new()
}
