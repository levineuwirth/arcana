//! Invasion of New Phyrexia // Teferi Akosa of Zhalfir
//!
//! Front: `{X}{W}{U}` Battle — Siege with 5 defense counters.
//! ETB: create X 2/2 white and blue Knight creature tokens with Vigilance.
//! GAP: ETB "create X tokens" — X from the cast is not exposed in the trigger
//! handler; returns Vec::new().
//!
//! Back: Legendary Planeswalker — Teferi (starting loyalty 4).
//! +1: Draw two cards. Then discard two cards unless you discard a creature card.
//! GAP: +1 "discard two unless you discard a creature card" — conditional
//!      discard shape not expressible.
//! −2: You get an emblem with "Knights you control get +1/+0 and have ward {1}."
//! GAP: Emblem creation not in Effect catalog.
//! −3: Tap any number of untapped creatures you control. Shuffle target nonland
//!      permanent an opponent controls with mana value ≤ X (creatures tapped) into
//!      its owner's library.
//! GAP: −3 multi-step tap + shuffle not modeled.
//!
//! GAP: defeat→cast-back-face not auto-wired (CR 310.11 deferred).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of New Phyrexia");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Teferi Akosa of Zhalfir (Legendary Planeswalker)
    let back_name = reg.interner_mut().intern("Teferi Akosa of Zhalfir");
    let teferi_sub = reg.interner_mut().intern("Teferi");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(teferi_sub);
    let back_chars = Characteristics {
        name: back_name,
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes: back_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };
    let back = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 5,
            })
            .with_transform_back(back)
            // ETB: create X 2/2 white and blue Knight tokens with Vigilance
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_resolve,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back-face +1
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Draw two cards. Then discard two cards unless you discard a creature card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: teferi_plus_one,
            })
            // Back-face -2
            .with_activated_ability(ActivatedAbilityDef {
                text: "\u{2212}2: You get an emblem with 'Knights you control get +1/+0 and have ward {1}.'".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: teferi_minus_two,
            })
            // Back-face -3
            .with_activated_ability(ActivatedAbilityDef {
                text: "\u{2212}3: Tap any number of untapped creatures you control. Shuffle target nonland permanent an opponent controls with mv \u{2264} X into its owner's library.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: teferi_minus_three,
            })
    )
}

fn etb_resolve(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: ETB "create X 2/2 white/blue Knight tokens with Vigilance" —
    // X from the Battle's cast is not exposed in the trigger handler.
    Vec::new()
}

fn teferi_plus_one(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "+1 draw two, discard two unless you discard a creature card" —
    // the conditional-discard form is not expressible.
    Vec::new()
}

fn teferi_minus_two(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Emblem creation not in Effect catalog.
    Vec::new()
}

fn teferi_minus_three(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "tap X untapped creatures you control; shuffle target nonland
    // permanent with mv ≤ X into library" — not expressible.
    Vec::new()
}
