//! Trystan, Callous Cultivator // Trystan, Penitent Culler —
//! `{2}{G}` Legendary Elf Druid 3/4 creature (front) with Deathtouch.
//!
//! Front face triggers:
//! - When this creature enters or transforms into Trystan, Callous Cultivator,
//!   mill three cards. Then if there is an Elf card in your graveyard, gain 2 life.
//! - At the beginning of your first main phase, you may pay {B}. If you do,
//!   transform Trystan.
//!
//! Back face: Legendary Elf Warlock with Deathtouch.
//! Back face triggers:
//! - When this creature transforms into Trystan, Penitent Culler, mill three
//!   cards, then you may exile an Elf card from your graveyard. If you do,
//!   each opponent loses 2 life.
//! - At the beginning of your first main phase, you may pay {G}. If you do,
//!   transform Trystan.
//!
//! # GAP
//! - Front ETB/transforms trigger: "if there is an Elf card in your graveyard"
//!   conditional is not expressible (no graveyard-card-type check in script).
//!   Modeled as unconditional mill 3 + gain 2 life (best effort).
//! - "Enters or transforms into" front trigger: ETB wired via SelfEntersBattlefield;
//!   "transforms into Trystan, Callous Cultivator" (transform-to-front) not
//!   separately expressible as a distinct trigger.
//! - Back transform trigger: "you may exile an Elf card from your graveyard.
//!   If you do, each opponent loses 2 life" — the optional exile-a-specific-card
//!   from graveyard gate requires OptionalPaymentKind::ExileFromGraveyard (GAP).
//!   Modeled as unconditional mill 3 for the back face (life-loss GAP'd).
//! - Back-face-only triggered ability not auto-installed; authored on
//!   CardDefinition — fires on both faces.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::targets::ControllerConstraint;
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trystan, Callous Cultivator");
    let elf_sub = reg.interner_mut().intern("Elf");
    let druid_sub = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf_sub);
    subtypes.0.insert(druid_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Trystan, Penitent Culler");
    let elf_sub2 = reg.interner_mut().intern("Elf");
    let warlock_sub = reg.interner_mut().intern("Warlock");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(elf_sub2);
    back_subtypes.0.insert(warlock_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Deathtouch],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Trigger 1: ETB — "when this creature enters" (models the "enters" half of
            // "enters or transforms into Trystan, Callous Cultivator")
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: front_enters_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 2: Front face — at beginning of first main phase, may pay {B} to transform
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: front_main_phase_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 3: Back face only — at beginning of first main phase, may pay {G} to transform back
            // GAP: back-face-only triggered ability — fires on both faces
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: back_main_phase_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn front_enters_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "mill three cards. Then if there is an Elf card in your graveyard, you gain 2 life."
    // GAP: "if there is an Elf card in your graveyard" conditional not expressible.
    // Modeling as mill 3 + gain 2 life unconditionally (best effort).
    vec![
        Effect::Mill { player: trig.controller, count: 3 },
        Effect::GainLife { player: trig.controller, amount: 2 },
    ]
}

fn front_main_phase_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "At the beginning of your first main phase, you may pay {B}. If you do, transform Trystan."
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{B}").expect("valid cost")),
        then: Box::new(Effect::Transform { target: trig.source }),
        else_effect: None,
    }]
}

fn back_main_phase_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "At the beginning of your first main phase, you may pay {G}. If you do, transform Trystan."
    // GAP: back-face-only triggered ability — this fires on both faces.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{G}").expect("valid cost")),
        then: Box::new(Effect::Transform { target: trig.source }),
        else_effect: None,
    }]
}
