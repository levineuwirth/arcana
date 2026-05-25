//! Arrogant Poet — `{1}{B}` 2/1 Human Warlock. "Whenever this
//! creature attacks, you may pay 2 life. If you do, it gains flying
//! until end of turn." Self-attack trigger with an optional life-
//! payment rider.
//!
//! GAP: the "you may pay 2 life. If you do, …" optional cost is not
//! expressible with the current catalog (no MayPay / OptionalCost
//! Effect). Resolved here as the unconditional "pay 2 life, gain
//! flying EOT" branch — a human review pass will need to gate the
//! payment on a controller choice once an optional-cost primitive
//! exists.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Arrogant Poet");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack_pay_for_flying,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// On attack, pay 2 life and grant flying until end of turn.
///
/// GAP: the optional "you may pay" choice is not modeled; this
/// resolver always pays. See file-level GAP note.
fn on_attack_pay_for_flying(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::LoseLife { player: trig.controller, amount: 2 },
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        },
    ]
}
