//! Krovikan Sorcerer — `{2}{U}` 1/1 Human Wizard Sorcerer.
//!
//! Oracle:
//! * {T}, Discard a nonblack card: Draw a card.
//! * {T}, Discard a black card: Draw two cards, then discard one of them.
//!
//! Both abilities pay {T} plus a chosen hand-card discard (the
//! `discard_other` cost with a color-filtered ObjectFilter — nonblack and
//! black respectively). The engine enumerates one activation per matching
//! hand card and handles the discard as a cost.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Krovikan Sorcerer");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let sorcerer = reg.interner_mut().intern("Sorcerer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    subtypes.0.insert(sorcerer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let nonblack = ObjectFilter::new().without_colors(ColorSet::black());
    let black = ObjectFilter::new().with_colors(ColorSet::black());

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Discard a nonblack card: Draw a card.".into(),
                cost: ActivationCost {
                    tap: true,
                    discard_other: Some(nonblack),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Discard a black card: Draw two cards, then discard one of them.".into(),
                cost: ActivationCost {
                    tap: true,
                    discard_other: Some(black),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_two_discard_one,
            }),
    )
}

/// "Draw a card."
fn draw_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }]
}

/// "Draw two cards, then discard one of them."
fn draw_two_discard_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards {
            player: ctx.controller,
            count: 2,
        },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
