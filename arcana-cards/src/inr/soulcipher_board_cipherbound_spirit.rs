//! Soulcipher Board // Cipherbound Spirit — `{1}{U}` Artifact that enters with
//! three omen counters on it.
//! `{1}{U}, {T}`: Look at the top two cards of your library. Put one of them
//! into your graveyard.
//! Whenever a creature card is put into your graveyard from anywhere, remove an
//! omen counter from this artifact. Then if it has no omen counters on it,
//! transform it.
//!
//! Back face: Cipherbound Spirit — `U` 3/2 Spirit with flying; can block only
//! creatures with flying; `{3}{U}: Draw two cards, then discard a card.`
//!
//! Transform DFC (CR 712). The front-face activated ability and the
//! omen-counter trigger are gated to face 0; the back draw ability to face 1.
//!
//! GAP: "Look at the top two ... put one into your graveyard" (a choose-one
//! mill) has no exact catalog variant — best-effort `Mill { count: 1 }`.
//! GAP: "then if it has no omen counters on it, transform it" — no Condition
//! variant tests source counter count; following the Ludevic's Test Subject
//! precedent we emit RemoveCounters + Transform and lose the count gate.
//! GAP: "can block only creatures with flying" static restriction not modeled.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soulcipher Board");
    let omen = reg.interner_mut().intern("omen");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };

    // Back face: Cipherbound Spirit — U 3/2 Spirit, Flying.
    let back_name = reg.interner_mut().intern("Cipherbound Spirit");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(spirit_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Named(omen),
                count: 3,
            })
            // Front-only: {1}{U}, {T}: look at top two, put one into graveyard.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{U}, {T}: Look at the top two cards of your library. Put one of them into your graveyard.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{U}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: dig_activate,
            })
            // Back-only: {3}{U}: Draw two cards, then discard a card.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}: Draw two cards, then discard a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: draw_activate,
            })
            // Front-only trigger: a creature card hits your graveyard → remove
            // an omen counter, then (GAP gate) transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature(),
                    from: None,
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: omen_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            .with_trigger_face_gate(1, 0),
    )
}

fn dig_activate(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "look at top two, put one into your graveyard" is a choose-one mill;
    // best-effort mill 1.
    vec![Effect::Mill { player: ctx.controller, count: 1 }]
}

fn draw_activate(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: ctx.controller, count: 2 },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}

fn omen_trigger(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let omen = reg
        .interner()
        .lookup("omen")
        .expect("omen interned during register");
    vec![
        Effect::RemoveCounters {
            target: trig.source,
            kind: CounterKind::Named(omen),
            count: 1,
        },
        // GAP: should only transform when no omen counters remain; no Condition
        // variant tests source counter count (mirrors Ludevic's Test Subject).
        Effect::Transform { target: trig.source },
    ]
}
