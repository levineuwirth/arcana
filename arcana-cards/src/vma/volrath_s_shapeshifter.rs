//! Volrath's Shapeshifter — `{1}{U}{U}` 0/1 Phyrexian Shapeshifter.
//!
//! * As long as the top card of your graveyard is a creature card, this
//!   creature has the full text of that card and has "{2}: Discard a card."
//!   (GAP — copy-the-graveyard-top static)
//! * {2}: Discard a card.
//!
//! The copy-the-top-graveyard-creature static (gain that card's name, cost,
//! color, types, abilities, P/T) has no expressible primitive and is GAP'd.
//! The "{2}: Discard a card." activated ability is wired.

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
    let name = reg.interner_mut().intern("Volrath's Shapeshifter");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "As long as the top card of your graveyard is a creature card, this
    //       creature has the full text of that card …" — copy-the-graveyard-top
    //       static (name/cost/color/types/abilities/P/T) is not expressible.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}: Discard a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: discard_a_card,
            }),
    )
}

fn discard_a_card(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Discard {
        player: ctx.controller,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
