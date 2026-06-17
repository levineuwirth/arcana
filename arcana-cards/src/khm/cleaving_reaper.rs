//! Cleaving Reaper — `{3}{B}{B}` 5/3 Angel Berserker with Flying and Trample.
//! "Pay 3 life: Return this card from your graveyard to your hand. Activate only if
//!  you had an Angel or Berserker enter the battlefield under your control this turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cleaving Reaper");
    let angel = reg.interner_mut().intern("Angel");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Pay 3 life: Return this card from your graveyard to your hand. Activate only if you had an Angel or Berserker enter the battlefield under your control this turn.".into(),
                cost: ActivationCost {
                    life: 3,
                    // GAP: "Activate only if you had an Angel or Berserker enter the
                    //   battlefield under your control this turn" — no documented
                    //   activation_condition predicate for "a creature of subtype X
                    //   entered under your control this turn".
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: return_self_to_hand,
            }),
    )
}

fn return_self_to_hand(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToHand { target: ctx.source }]
}
