//! The Great Juggernaut — `{3}{R}` 5/3 Legendary Artifact Creature —
//! Juggernaut.
//! "At the beginning of your upkeep, sacrifice The Great Juggernaut
//! unless you discard a card. Whenever The Great Juggernaut attacks,
//! shuffle your library then exile the top card of your library. You may
//! play that card without paying its mana cost this turn."
//!
//! No keyword line. Trigger 1 (upkeep "sacrifice unless you discard a
//! card") is a discard-payment gate; OptionalPaymentKind carries only
//! Mana/Life in v1, so "unless you discard a card" is not an expressible
//! cost — GAP the effect. Trigger 2 is an attack impulse: the closest
//! primitive is Effect::ImpulseExile of one card (exile the top card,
//! may play it this turn). FIDELITY GAP: ImpulseExile lets you play at
//! normal cost, whereas the oracle is "without paying its mana cost";
//! the pre-exile shuffle is also un-modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Great Juggernaut");
    let juggernaut = reg.interner_mut().intern("Juggernaut");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(juggernaut);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_sacrifice_unless_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_impulse,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_sacrifice_unless_discard(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "sacrifice ~ unless you discard a card" — discard is not an
    // OptionalPaymentKind, so the unless-discard gate is unexpressible.
    Vec::new()
}

fn attack_impulse(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // FIDELITY GAP: oracle plays the exiled card "without paying its mana
    // cost"; ImpulseExile permits casting at normal cost. Pre-exile shuffle un-modeled.
    vec![Effect::ImpulseExile { player: trig.controller, count: 1 }]
}
