//! Despoiler of Souls — `{B}{B}` 3/1 Horror.
//! "This creature can't block." — GAP (a permanent self-static "can't
//! block" has no expressible attachment point in this card class — the
//! catalog's ForbidBlocking is a targeted until-end-of-turn effect, not
//! a continuous self-static, and there is no static-ability slot).
//! "{B}{B}, Exile two other creature cards from your graveyard: Return
//! this card from your graveyard to the battlefield." — the exile cost
//! component is not an expressible ActivationCost field (no
//! exile-from-graveyard cost), so only the {B}{B} cost is modeled and
//! the exile cost is GAP-noted; the graveyard-activated return is wired.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Despoiler of Souls");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            // GAP: "Exile two other creature cards from your graveyard"
            // cost component is not an expressible ActivationCost field.
            text: "{B}{B}, Exile two other creature cards from your graveyard: Return this card from your graveyard to the battlefield.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{B}{B}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: return_self_from_graveyard,
        }),
    )
}

fn return_self_from_graveyard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToBattlefield { target: ctx.source }]
}
