//! Bomat Courier — `{1}` 1/1 Artifact Creature — Construct (colorless) with Haste.
//! Whenever this creature attacks, exile the top card of your library face down.
//! {R}, Discard your hand, Sacrifice this creature: Put all cards exiled with
//! this creature into their owners' hands.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bomat Courier");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: exile_top_face_down,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}, Discard your hand, Sacrifice this creature: Put all cards exiled with this creature into their owners' hands.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                    discard_hand: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: return_exiled,
            }),
    )
}

/// Exile the top card of the library face down, linked to this creature.
fn exile_top_face_down(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::ExileTopFaceDownLinked { player: trig.controller, link_to: trig.source }]
}

/// Each card exiled with this creature goes to its owner's hand. The cards
/// are those linked to the creature as it was on the battlefield, which
/// the sacrifice paid as a cost does not change (CR 607.2a).
fn return_exiled(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let mut linked: Vec<_> = state.objects.iter()
        .filter(|o| o.zone.is_exile() && o.linked_exile_source == Some(ctx.source))
        .map(|o| o.id)
        .collect();
    linked.sort_unstable();
    linked.into_iter().map(|target| Effect::ReturnToHand { target }).collect()
}
