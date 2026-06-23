//! Waker of Waves — `{5}{U}{U}` 7/7 Whale (U).
//!
//! Oracle:
//! * Creatures your opponents control get -1/-0. (static anthem-debuff —
//!   GAP).
//! * {1}{U}, Discard this card: Look at the top two cards of your
//!   library. Put one of them into your hand and the other into your
//!   graveyard.
//!
//! The opponents'-creatures debuff is a pure continuous static, not
//! expressible on this card class — GAP. The hand-activated dig is
//! wired: a mana + discard-self cost activated from the hand, looking at
//! the top two cards (take one to hand, the other to graveyard) via
//! `DigTopN` with `DigRest::Graveyard`.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Waker of Waves");
    let whale = reg.interner_mut().intern("Whale");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(whale);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "Creatures your opponents control get -1/-0." (anthem-debuff).
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{U}, Discard this card: Look at the top two cards of \
                       your library. Put one of them into your hand and the \
                       other into your graveyard."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{U}").expect("valid cost"),
                    discard_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Hand,
                is_instant_speed: false,
                face_gate: None,
                effect: dig_two,
            }),
    )
}

fn dig_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 2,
        filter: None,
        rest: DigRest::Graveyard,
    }]
}
