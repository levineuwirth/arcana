//! Brigid, Clachan's Heart // Brigid, Doun's Mind — `{2}{W}` Legendary Kithkin Warrior 3/2.
//! Front: Whenever this enters or transforms into Brigid, Clachan's Heart, create a
//! 1/1 green and white Kithkin creature token.
//! Front: At the beginning of your first main phase, you may pay {G}. If you do, transform.
//! Back (Brigid, Doun's Mind): Legendary Kithkin Soldier.
//! Back: {T}: Add X {G} or X {W}, where X is the number of other creatures you control.
//! Back: At the beginning of your first main phase, you may pay {W}. If you do, transform.
//!
//! GAP: Back-face {T} mana ability (dynamic X, color choice) is a back-face-only activated
//! mana ability — not modeled here.
//! GAP: Back-face transform trigger is a back-face-only triggered ability not modeled.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brigid, Clachan's Heart");
    let kithkin_sub = reg.interner_mut().intern("Kithkin");
    let warrior_sub = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin_sub);
    subtypes.0.insert(warrior_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Brigid, Doun's Mind");
    let back_kithkin_sub = reg.interner_mut().intern("Kithkin");
    let back_soldier_sub = reg.interner_mut().intern("Soldier");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_kithkin_sub);
    back_subtypes.0.insert(back_soldier_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    // Pre-intern Kithkin for token (needed at resolve time via lookup)
    let _kithkin_tok = reg.interner_mut().intern("Kithkin");

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Trigger 1: ETB — create a 1/1 green and white Kithkin creature token.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 3: "or transforms into Brigid, Clachan's Heart" (front face)
            // half of the token trigger.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(0) },
                intervening_if: None,
                effect: etb_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 2: Front-face — at beginning of first main phase, may pay {G} to transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: front_transform_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn etb_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let kithkin_name = reg.interner().lookup("Kithkin").unwrap_or_default();
    let mut tok_subtypes = SubtypeSet::default();
    tok_subtypes.0.insert(kithkin_name);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: kithkin_name,
            colors: ColorSet::green() | ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: tok_subtypes,
            keywords: vec![],
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            abilities: vec![],
        },
    }]
}

fn front_transform_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "At the beginning of your first main phase, you may pay {G}. If you do, transform Brigid."
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{G}").expect("valid cost")),
        then: Box::new(Effect::Transform { target: trig.source }),
        else_effect: None,
    }]
}
