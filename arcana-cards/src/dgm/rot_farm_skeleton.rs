//! Rot Farm Skeleton — `{2}{B}{G}` 4/1 Plant Skeleton.
//!
//! This creature can't block. (GAP — static "can't block" with no
//!   triggered/activated hook; ForbidBlocking is a targeted EOT effect,
//!   not a permanent self-static.)
//! {2}{B}{G}, Mill four cards: Return this card from your graveyard to
//!   the battlefield. Activate only as a sorcery.
//!
//! The "Mill four cards" portion is an additional cost; ActivationCost
//! has no mill-cost field, so it is approximated as a resolution step
//! (mill then return) and noted as a fidelity GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rot Farm Skeleton");
    let plant = reg.interner_mut().intern("Plant");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(skeleton);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{B}{G}, Mill four cards: Return this card from your graveyard to the battlefield. Activate only as a sorcery.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{B}{G}").expect("valid cost"),
                // GAP: "Mill four cards" additional cost — no mill cost field.
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: mill_then_return_self,
        }),
    )
}

fn mill_then_return_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sequence(vec![
        Effect::Mill { player: ctx.controller, count: 4 },
        Effect::ReturnFromGraveyardToBattlefield { target: ctx.source },
    ])]
}
