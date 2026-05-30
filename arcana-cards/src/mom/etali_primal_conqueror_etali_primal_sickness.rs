//! Etali, Primal Conqueror // Etali, Primal Sickness — `{5}{R}{R}` Legendary Elder Dinosaur 7/7.
//!
//! Front face (Etali, Primal Conqueror):
//! - Trample
//! - When Etali enters, each player exiles cards from the top of their library until they exile
//!   a nonland card. You may cast any number of spells from among the nonland cards exiled this way
//!   without paying their mana costs.
//!   GAP: "each player exiles until nonland, cast any number for free" — mass per-player RevealUntil
//!   with free-cast of multiple exiled cards is not expressible. Emitting Vec::new().
//! - {9}{G/P}: Transform Etali. Activate only as a sorcery.
//!
//! Back face (Etali, Primal Sickness): Legendary Phyrexian Elder Dinosaur, Trample + Indestructible.
//! - Whenever Etali deals combat damage to a player, they get that many poison counters.
//!   GAP: back-face-only triggered ability not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationContext, CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Etali, Primal Conqueror");
    let elder_sub = reg.interner_mut().intern("Elder");
    let dino_sub = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder_sub);
    subtypes.0.insert(dino_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        keywords: vec![KeywordAbility::Trample],
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };

    // Back face: Etali, Primal Sickness — Legendary Phyrexian Elder Dinosaur, Trample + Indestructible
    let back_name = reg.interner_mut().intern("Etali, Primal Sickness");
    let back_phyrexian = reg.interner_mut().intern("Phyrexian");
    let back_elder = reg.interner_mut().intern("Elder");
    let back_dino = reg.interner_mut().intern("Dinosaur");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_phyrexian);
    back_subtypes.0.insert(back_elder);
    back_subtypes.0.insert(back_dino);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            keywords: vec![KeywordAbility::Trample, KeywordAbility::Indestructible],
            power: Some(PtValue::Fixed(7)),
            toughness: Some(PtValue::Fixed(7)),
            ..Default::default()
        },
        spell_ability: None,
    };

    // ETB trigger: each player exiles until nonland, cast any number free.
    // GAP: not expressible.

    // {9}{G/P}: Transform. Activate only as a sorcery.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etali_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{9}{G/P}: Transform Etali. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{9}{G/P}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: transform_etali,
            }),
    )
}

fn etali_etb(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each player exiles cards from the top of their library until they exile a nonland card.
    // You may cast any number of spells from among the nonland cards exiled this way without paying
    // their mana costs." — mass per-player RevealUntil with multi-card free-cast is not expressible.
    Vec::new()
}

fn transform_etali(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
