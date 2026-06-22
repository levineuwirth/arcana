//! Qiqirn Merchant — `{2}{U}` 1/4 blue Beast Citizen.
//!
//! `{1}`, `{T}`: Draw a card, then discard a card.
//! `{7}`, `{T}`, Sacrifice this creature: Draw three cards. (This
//! ability costs {1} less to activate for each Town you control — the
//! cost reduction is GAP'd; the printed {7} cost is used.)

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Qiqirn Merchant");
    let beast = reg.interner_mut().intern("Beast");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    subtypes.0.insert(citizen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "{1}, {T}: Draw a card, then discard a card."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}: Draw a card, then discard a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: loot_one,
            })
            // "{7}, {T}, Sacrifice this creature: Draw three cards."
            // GAP: "costs {1} less for each Town you control" — no cost-
            // reduction field on ActivationCost; the printed {7} is used.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{7}, {T}, Sacrifice this creature: Draw three cards.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{7}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_three,
            }),
    )
}

fn loot_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sequence(vec![
        Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ])]
}

fn draw_three(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 3,
    }]
}
