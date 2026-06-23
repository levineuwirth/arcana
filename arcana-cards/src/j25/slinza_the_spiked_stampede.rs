//! Slinza, the Spiked Stampede — `{4}{G}` 5/5 Legendary Beast (green).
//!
//! Oracle:
//! * Beast spells you cast cost {2} less to cast. (GAP: cost-reduction
//!   static — no engine primitive.)
//! * Each other Beast creature you control enters with an additional +1/+1
//!   counter on it. (GAP: enters-with-counter static replacement — not
//!   expressible on a MultiAbilityCreature def.)
//! * Whenever Slinza or another creature with power 4 or greater enters, you
//!   may pay {1}{R/G}. When you do, Slinza fights target creature you don't
//!   control.
//!
//! The Scryfall "Fight" keyword is a parse of the reflexive fight clause,
//! not a `KeywordAbility` variant, so `keywords` is empty.
//!
//! Implemented:
//! * The ETB-power-4 reflexive trigger: a creature with power ≥ 4 (Slinza
//!   included) entering posts the optional `{1}{R/G}` payment; on payment
//!   Slinza fights the chosen creature you don't control.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slinza, the Spiked Stampede");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_min_power(4),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: maybe_pay_then_fight,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn maybe_pay_then_fight(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}{R/G}").expect("valid cost")),
        then: Box::new(Effect::Fight { a: trig.source, b: *id }),
        else_effect: None,
    }]
}
