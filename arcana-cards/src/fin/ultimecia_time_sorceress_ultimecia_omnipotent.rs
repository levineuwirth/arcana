//! Ultimecia, Time Sorceress // Ultimecia, Omnipotent — `{3}{U}{B}` Legendary
//! Creature — Human Warlock 4/5. Whenever she enters or attacks, surveil 2.
//! At beginning of your end step, you may pay {4}{U}{U}{B}{B} and exile eight
//! cards from your graveyard; if you do, transform Ultimecia.
//!
//! Back face: Legendary Creature — Nightmare Warlock with Menace.
//!
//! # GAP notes
//! - "Exile eight cards from your graveyard" as an additional cost alongside
//!   the mana payment is not expressible with OptionalPaymentKind (only Mana/Life).
//!   The exile-eight requirement is omitted; only the mana payment is modeled.
//! - "Take an extra turn after this one" (back face Time Compression trigger) has
//!   no Effect::ExtraTurn variant. GAP: back-face-only triggered ability not modeled.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ultimecia, Time Sorceress");
    let human_sub = reg.interner_mut().intern("Human");
    let warlock_sub = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(warlock_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ultimecia, Omnipotent");
    let nightmare_sub = reg.interner_mut().intern("Nightmare");
    let warlock_back = reg.interner_mut().intern("Warlock");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(nightmare_sub);
    back_subtypes.0.insert(warlock_back);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue() | ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(7)),
            toughness: Some(PtValue::Fixed(7)),
            keywords: vec![KeywordAbility::Menace],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Trigger 1: whenever Ultimecia enters, surveil 2.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: surveil_2_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Trigger 2: whenever Ultimecia attacks, surveil 2.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: surveil_2_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Trigger 3: at beginning of your end step, optionally pay to transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            }),
    )
}

fn surveil_2_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Surveil {
        player: trig.controller,
        count: 2,
    }]
}

fn end_step_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile eight cards from your graveyard" requirement is not expressible
    // with OptionalPaymentKind (only Mana/Life); only the mana payment is modeled.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{4}{U}{U}{B}{B}").expect("valid cost"),
        ),
        then: Box::new(Effect::Transform { target: trig.source }),
        else_effect: None,
    }]
}
