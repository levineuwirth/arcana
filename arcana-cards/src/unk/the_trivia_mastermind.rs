//! The Trivia Mastermind — `{1}{U}{B}{R}` 1/4 Legendary Human Gamer.
//! {1}, {T}: Look at the top three cards of your library. Choose one and an
//! opponent guesses a category about it; if they guess wrong, put it into your
//! hand. Put the rest on the bottom in any order.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Trivia Mastermind");
    let human = reg.interner_mut().intern("Human");
    let gamer = reg.interner_mut().intern("Gamer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(gamer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}: Look at the top three cards of your library. \
                       Choose one, an opponent guesses a category; if wrong put \
                       it into your hand. Put the rest on the bottom."
                    .into(),
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
                effect: trivia_dig,
            }),
    )
}

// Best-effort: model the look-at-top-three / take-one-to-hand / rest-to-bottom
// shape via DigTopN. GAP: the category guessing mini-game (opponent guesses
// power/toughness/mana value, conditional take only on a wrong guess) is not
// expressible — this approximates with an unconditional may-take.
fn trivia_dig(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 3,
        filter: None,
        rest: DigRest::BottomRandom,
    }]
}
