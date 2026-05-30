//! Restless Bloodseeker // Bloodsoaked Reveler
//!
//! Front: Creature — Vampire {1}{B}, 1/3
//! At the beginning of your end step, if you gained life this turn, create a Blood token.
//! Sacrifice two Blood tokens: Transform this creature. Activate only as a sorcery.
//! (GAP: "Sacrifice two Blood tokens" as an activated ability cost not modeled —
//!  SacrificeOther cost not in OptionalPaymentKind. The transform is not triggered here.)
//! (GAP: "if you gained life this turn" condition not checkable; emitting unconditionally.)
//!
//! Back (Bloodsoaked Reveler): Creature — Vampire
//! At the beginning of your end step, if you gained life this turn, create a Blood token.
//! {4}{B}: Each opponent loses 2 life and you gain 2 life.
//! GAP: back-face-only activated ability ({4}{B}) not modeled.
//! GAP: back-face-only triggered ability not modeled.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::targets::ControllerConstraint;
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Restless Bloodseeker");

    let vampire = reg.interner_mut().intern("Vampire");
    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(vampire);

    // Pre-intern Blood subtype for resolver use
    let _blood = reg.interner_mut().intern("Blood");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Bloodsoaked Reveler");
    let vampire2 = reg.interner_mut().intern("Vampire");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vampire2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face trigger: at beginning of your end step, if you gained life this turn,
            // create a Blood token.
            // GAP: "if you gained life this turn" condition not checkable; emitting unconditionally.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_blood_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: "Sacrifice two Blood tokens: Transform" activated ability cost not modeled.
            // GAP: back-face-only activated ability {4}{B} not modeled.
            // GAP: back-face-only triggered ability not modeled.
    )
}

fn end_step_blood_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let blood = reg.interner().lookup("Blood").expect("Blood interned during register()");
    let mut blood_subtypes = SubtypeSet::default();
    blood_subtypes.0.insert(blood);
    let token = TokenDefinition {
        name: blood,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: blood_subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}
