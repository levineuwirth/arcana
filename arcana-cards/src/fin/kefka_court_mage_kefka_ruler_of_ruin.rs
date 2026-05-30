//! Kefka, Court Mage // Kefka, Ruler of Ruin — `{2}{U}{B}{R}` Legendary Creature — Human Wizard 4/5 (U/B/R).
//! Flying.
//! Whenever Kefka enters or attacks, each player discards a card. Then you draw a card for each
//!   card type among cards discarded this way.
//! {8}: Each opponent sacrifices a permanent of their choice. Transform Kefka. (Sorcery speed.)
//! Back face: Kefka, Ruler of Ruin — Legendary Creature — Avatar Wizard, Flying.
//!   Whenever an opponent loses life during your turn, you draw that many cards.
//!
//! GAP: "draw a card for each card type among cards discarded this way" — per-discard card-type
//!   tracking not expressible; only the discard part fires, no draw.
//! GAP: Back-face-only triggered ability ("whenever an opponent loses life during your turn,
//!   you draw that many cards") not modeled (back-face triggers not auto-installed).

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kefka, Court Mage");
    let human_sub = reg.interner_mut().intern("Human");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut front_subs = SubtypeSet::default();
    front_subs.0.insert(human_sub);
    front_subs.0.insert(wizard_sub);

    let front_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subs,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Kefka, Ruler of Ruin");
    let avatar_sub = reg.interner_mut().intern("Avatar");
    let wizard_back_sub = reg.interner_mut().intern("Wizard");
    let mut back_subs = SubtypeSet::default();
    back_subs.0.insert(avatar_sub);
    back_subs.0.insert(wizard_back_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subs,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Flying],
            // GAP: back-face-only triggered ability "whenever an opponent loses life
            // during your turn, you draw that many cards" not modeled.
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, front_chars)
            .with_transform_back(back)
            // Trigger 1: Whenever Kefka enters
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: enters_or_attacks_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 2: Whenever Kefka attacks
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: enters_or_attacks_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // {8}: Each opponent sacrifices a permanent; transform. Sorcery speed.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{8}: Each opponent sacrifices a permanent of their choice. Transform Kefka. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{8}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: eight_activate,
            }),
    )
}

fn enters_or_attacks_discard(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Each player discards a card.
    let mut effects: Vec<Effect> = script::all_players(state)
        .into_iter()
        .map(|p| Effect::Discard {
            player: p,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        })
        .collect();
    // GAP: "draw a card for each card type among cards discarded this way" —
    // per-discard card-type tracking not expressible in current engine.
    let _ = trig.controller;
    effects
}

fn eight_activate(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = script::opponents(state, ctx.controller)
        .into_iter()
        .map(|p| Effect::Sacrifice {
            player: p,
            filter: ObjectFilter::permanent(),
            count: 1,
        })
        .collect();
    effects.push(Effect::Transform { target: ctx.source });
    effects
}
