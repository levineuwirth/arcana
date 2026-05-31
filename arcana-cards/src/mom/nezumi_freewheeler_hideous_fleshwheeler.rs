//! Nezumi Freewheeler // Hideous Fleshwheeler (transforming DFC, layout "transform")
//!
//! Front face: Nezumi Freewheeler — {3}{B} Creature — Rat Samurai, 3/3 (B).
//!   Menace.
//!   When this creature enters, each player mills three cards.
//!   {5}{W/P}: Transform this creature. Activate only as a sorcery.
//! Back face: Hideous Fleshwheeler — Creature — Phyrexian Rat, 3/3 (B).
//!   Menace.
//!   When this creature transforms into Hideous Fleshwheeler, put target permanent card with
//!     mana value 2 or less from a graveyard onto the battlefield under your control.
//!
//! GAP: the back-face "When this creature transforms into Hideous Fleshwheeler, ..." trigger
//!   is not expressible — there is no transform-completion TriggerCondition in the demonstrated
//!   API (only StepBegins/ZoneChange/etc.). The ETB each-player-mills-3 trigger and the
//!   sorcery-speed activated transform ({5}{W/P}) ARE authored.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardFace, CardRegistry,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nezumi Freewheeler");
    let rat = reg.interner_mut().intern("Rat");
    let samurai = reg.interner_mut().intern("Samurai");
    let phyrexian = reg.interner_mut().intern("Phyrexian");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(rat);
    front_subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Hideous Fleshwheeler");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(phyrexian);
    back_subtypes.0.insert(rat);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Menace],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front ETB: each player mills three cards. Front-only.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_mill,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Front: {5}{W/P}: Transform this creature. Sorcery speed. Front-only.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{W/P}: Transform this creature. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{W/P}").expect("valid cost"),
                    ..Default::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: transform_self,
            })
            .with_trigger_face_gate(1, 0),
    )
}

fn etb_mill(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let players = script::all_players(state);
    let mills: Vec<Effect> = players
        .into_iter()
        .map(|p| Effect::Mill { player: p, count: 3 })
        .collect();
    let _ = trig;
    vec![Effect::Sequence(mills)]
}

fn transform_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
