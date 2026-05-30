//! Aang, Swift Savior // Aang and La, Ocean's Fury
//!
//! Front face: {1}{W}{U} Legendary Creature — Human Avatar Ally 2/3, Flash, Flying.
//!   When Aang enters, airbend up to one other target creature or spell. (Exile it.
//!   While it's exiled, its owner may cast it for {2} rather than its mana cost.)
//!   Waterbend {8}: Transform Aang.
//!
//! Back face: Legendary Creature — Avatar Spirit Ally, Reach, Trample.
//!   Whenever Aang and La attack, put a +1/+1 counter on each tapped creature you
//!   control.
//!
//! GAP: "Airbend" (exile target and allow casting for {2}) is not a modeled engine
//! Effect; the ETB trigger emits no-op.
//! GAP: "Waterbend" is not a modeled keyword; the {8} transform activated ability
//! is authored as a generic mana-cost activation.
//! GAP: back-face attack trigger "put a +1/+1 counter on each tapped creature you
//! control" not modeled (back-face-only triggered ability not auto-installed).
//! GAP: back-face P/T not printed in Scryfall data; best-effort 5/5.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aang, Swift Savior");
    let human_sub = reg.interner_mut().intern("Human");
    let avatar_sub = reg.interner_mut().intern("Avatar");
    let ally_sub = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(avatar_sub);
    subtypes.0.insert(ally_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Aang and La, Ocean's Fury");
    let avatar_back_sub = reg.interner_mut().intern("Avatar");
    let spirit_back_sub = reg.interner_mut().intern("Spirit");
    let ally_back_sub = reg.interner_mut().intern("Ally");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(avatar_back_sub);
    back_subtypes.0.insert(spirit_back_sub);
    back_subtypes.0.insert(ally_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white() | ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Reach, KeywordAbility::Trample],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB trigger: airbend up to one target creature or spell.
            // GAP: "Airbend" effect (exile + may cast for {2}) not modeled; no-op.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_airbend,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Waterbend {8}: Transform Aang. Sorcery speed.
            .with_activated_ability(ActivatedAbilityDef {
                text: "Waterbend {8}: Transform this creature. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{8}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: waterbend_transform,
            }),
        // GAP: back-face attack trigger (put +1/+1 counters on tapped creatures) not modeled.
    )
}

fn etb_airbend(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Airbend" — exile target creature or spell; while exiled its owner may
    // cast it for {2} rather than its mana cost. No engine Effect for this. No-op.
    Vec::new()
}

fn waterbend_transform(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
