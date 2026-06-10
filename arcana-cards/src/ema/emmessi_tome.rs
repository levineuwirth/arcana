//! Emmessi Tome — `{4}` artifact — Book (Fifth Edition).
//! "{5}, {T}: Draw two cards, then discard a card."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Emmessi Tome");
    let book = reg.interner_mut().intern("Book");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(book);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{5}, {T}: Draw two cards, then discard a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_two_discard_one,
            },
        ),
    )
}

fn draw_two_discard_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: ctx.controller, count: 2 },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
