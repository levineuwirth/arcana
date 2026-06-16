//! Nether Traitor — `{B}{B}` 1/1 black Spirit with Haste and Shadow.
//! "Whenever another creature is put into your graveyard from the
//! battlefield, you may pay {B}. If you do, return this card from your
//! graveyard to the battlefield."
//!
//! Decomposition:
//! * Keyword line: Haste, Shadow.
//! * One triggered ability — a creature you control dying (battlefield →
//!   graveyard) lets you pay {B} to recur this card from the graveyard.
//!   Modeled as `TriggerCondition::ZoneChange` over a creature you control
//!   moving from battlefield to graveyard, with the trigger active from the
//!   graveyard zone (where Nether Traitor lives when it can recur). The
//!   "you may pay {B}. If you do, return …" gate is an `Effect::OptionalPayment`
//!   wrapping `Effect::ReturnFromGraveyardToBattlefield` of the source.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nether Traitor");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste, KeywordAbility::Shadow],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: maybe_recur,
            trigger_zones: vec![Zone::Graveyard(0)],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "you may pay {B}. If you do, return this card from your graveyard to the
/// battlefield."
fn maybe_recur(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{B}").expect("valid cost")),
        then: Box::new(Effect::ReturnFromGraveyardToBattlefield { target: trig.source }),
        else_effect: None,
    }]
}
