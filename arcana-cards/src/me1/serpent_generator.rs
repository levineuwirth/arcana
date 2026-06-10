//! Serpent Generator — `{6}` artifact.
//! "{4}, {T}: Create a 1/1 colorless Snake artifact creature token.
//! It has 'Whenever this creature deals damage to a player, that
//! player gets a poison counter.'"
//!
//! GAP: the token's printed triggered ability ("that player gets a
//! poison counter") cannot be attached to a `TokenDefinition`
//! (`abilities` carries no authored triggered abilities here) — the
//! bare 1/1 Snake artifact creature token is minted without the
//! poison rider.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Serpent Generator");
    // Pre-intern the token subtype for the resolver's read-only lookup.
    let _snake = reg.interner_mut().intern("Snake");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{4}, {T}: Create a 1/1 colorless Snake artifact \
                       creature token. It has \"Whenever this creature \
                       deals damage to a player, that player gets a poison \
                       counter.\""
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_snake,
            },
        ),
    )
}

fn make_snake(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the token's "Whenever this creature deals damage to a player,
    // that player gets a poison counter" triggered ability is not
    // expressible on a TokenDefinition — bare token only.
    let snake = reg.interner().lookup("Snake").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: snake,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
