//! Teacher's Pest — `{B}{G}` 1/1 Skeleton Pest with Menace.
//! Whenever this creature attacks, you gain 1 life.
//! `{B}{G}: Return this card from your graveyard to the battlefield tapped.`

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
    let name = reg.interner_mut().intern("Teacher's Pest");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let pest = reg.interner_mut().intern("Pest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(skeleton);
    subtypes.0.insert(pest);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: gain_one_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}{G}: Return this card from your graveyard to the battlefield tapped."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: reanimate_self_tapped,
            }),
    )
}

fn gain_one_life(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainLife { player: trig.controller, amount: 1 }]
}

fn reanimate_self_tapped(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return THIS card from your graveyard to the battlefield TAPPED" —
    // no self-targeting graveyard→battlefield-tapped Effect is available
    // (ReturnFromGraveyardToBattlefield targets a chosen card and does not
    // tap). Activation cost + zone are modeled; the move itself is the gap.
    let _ = ctx;
    Vec::new()
}
