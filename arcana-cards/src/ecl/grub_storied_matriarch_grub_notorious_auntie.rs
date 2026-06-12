//! Grub, Storied Matriarch // Grub, Notorious Auntie — `{2}{B}` Legendary
//! Creature — Goblin Warlock 2/1 (front), transforms to Legendary Creature —
//! Goblin Warrior (back).
//!
//! Front: Menace. Whenever this enters or transforms into Grub, Storied
//! Matriarch, return up to one target Goblin card from your graveyard to your
//! hand. At the beginning of your first main phase, you may pay {R}. If you do,
//! transform Grub.
//!
//! Back: Menace. Whenever Grub attacks, you may blight 1 (not modeled).
//! At the beginning of your first main phase, you may pay {B}. If you do,
//! transform Grub back.
//!
//! # GAP notes
//! - Keywords "Blight" and "Transform" (Scryfall marker) are not in the
//!   KeywordAbility enum; omitted.
//! - Back-face attack trigger (Blight 1 + copy-token) not modeled.
//!   // GAP: back-face-only triggered ability not modeled.
//! - The two PhaseBegins triggers (ids 2 and 3) both fire from either face;
//!   engine has no face-gate on TriggeredAbilityDef.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grub, Storied Matriarch");
    let goblin_sub = reg.interner_mut().intern("Goblin");
    let warlock_sub = reg.interner_mut().intern("Warlock");
    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(goblin_sub);
    front_subtypes.0.insert(warlock_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Grub, Notorious Auntie");
    let goblin_sub2 = reg.interner_mut().intern("Goblin");
    let warrior_sub = reg.interner_mut().intern("Warrior");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(goblin_sub2);
    back_subtypes.0.insert(warrior_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Menace],
            ..Default::default()
        },
        spell_ability: None,
    };

    // ETB filter: self entering (controlled_by You, creature type).
    let etb_filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);

    // Target requirement for ETB trigger: up to one Goblin card in your graveyard.
    let goblin_lookup = reg.interner().lookup("Goblin");
    let goblin_gy_req = TargetRequirement {
        filter: TargetFilter::Card {
            zone: Zone::Graveyard(0),
            filter: {
                let mut f = ObjectFilter::creature();
                if let Some(g) = goblin_lookup {
                    f = f.with_subtypes_any(vec![g]);
                }
                f
            },
        },
        count: TargetCount::UpTo(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // ETB half of "enters or transforms into" trigger.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: etb_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: etb_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![goblin_gy_req.clone()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4,
                // "Transforms into Grub, Storied Matriarch" (front face) half.
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(0) },
                intervening_if: None,
                effect: etb_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![goblin_gy_req],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // Front-face: beginning of first main phase, may pay {R}, transform.
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: front_phase_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                // Back-face: beginning of first main phase, may pay {B}, transform.
                // GAP: back-face-only triggered ability not modeled; fires from both faces.
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: back_phase_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}

fn front_phase_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{R}").expect("valid cost")),
        then: Box::new(Effect::Transform { target: trig.source }),
        else_effect: None,
    }]
}

fn back_phase_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Back-face: you may pay {B}. If you do, transform back.
    // GAP: back-face-only triggered ability not modeled.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{B}").expect("valid cost")),
        then: Box::new(Effect::Transform { target: trig.source }),
        else_effect: None,
    }]
}
