//! Darigaaz, the Igniter — `{3}{B}{R}{G}` 6/6 Legendary Dragon with
//! Flying.
//! "Whenever Darigaaz deals combat damage to a player, you may pay
//! {2}{R}. If you do, choose a color, then that player reveals their hand
//! and Darigaaz deals damage to the player equal to the number of cards
//! of that color revealed this way."
//!
//! Flying is a base keyword. The combat-damage trigger's optional {2}{R}
//! payment is modeled with OptionalPayment, but the "if you do" payoff —
//! choose a color, reveal hand, deal damage equal to the count of that
//! color — has no expressible primitive (no choose-color + reveal-count +
//! dynamic damage), so the `then` body is GAP'd (no-op).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Darigaaz, the Igniter");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: combat_damage_payoff,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn combat_damage_payoff(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{2}{R}").expect("valid cost")),
        // GAP: "choose a color, that player reveals their hand, deal damage
        // equal to the number of cards of that color revealed" — no choose-
        // color + reveal-count + dynamic-damage primitive.
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: None,
    }]
}
