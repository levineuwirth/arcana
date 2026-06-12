//! Captive Weird // Compleated Conjurer — `{U}` Weird creature 1/3 with
//! Defender. Front face: `{3}{R/P}`: Transform this creature (sorcery speed).
//! Back face (Compleated Conjurer): When this creature transforms into
//! Compleated Conjurer, exile the top card of your library; until end of your
//! next turn you may play that card.
//!
//! GAP: "until end of your NEXT turn you may play that card" — the impulse-exile
//! play window only lasts until end of the current turn (Effect::ImpulseExile);
//! the extra turn of play permission is not modeled.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Captive Weird");
    let weird_sub = reg.interner_mut().intern("Weird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(weird_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Compleated Conjurer");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let weird_back_sub = reg.interner_mut().intern("Weird");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(phyrexian_sub);
    back_subtypes.0.insert(weird_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front: {3}{R/P}: Transform this creature. Activate only as a sorcery.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{R/P}: Transform this creature. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{R/P}").expect("valid phyrexian cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: transform_self,
            })
            // Back: When this creature transforms into Compleated Conjurer, exile
            // the top card of your library. Until end of your next turn, you may
            // play that card.
            // GAP: play window is until end of THIS turn (ImpulseExile), not end
            // of your next turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: on_transform_impulse,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

fn on_transform_impulse(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ImpulseExile {
        player: trig.controller,
        count: 1,
    }]
}
