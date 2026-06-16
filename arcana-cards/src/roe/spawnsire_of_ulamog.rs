//! Spawnsire of Ulamog — `{10}` 7/11 Eldrazi.
//! Annihilator 1; "{4}: Create two 0/1 colorless Eldrazi Spawn tokens with
//! 'Sacrifice this token: Add {C}.'"; "{20}: Cast any number of Eldrazi spells
//! from outside the game without paying their mana costs."
//!
//! Annihilator is not in the usable KeywordAbility surface (GAP). The {4}
//! activated ability creates the two Spawn tokens (their printed sacrifice-for-
//! mana ability cannot be authored on a TokenDefinition here, so that token
//! ability is a GAP). The {20} "cast from outside the game" ability has no
//! expressible Effect (GAP'd, empty resolver).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::effects::TokenDefinition;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spawnsire of Ulamog");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    // GAP: Annihilator 1 is not in the usable KeywordAbility surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{10}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(11)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}: Create two 0/1 colorless Eldrazi Spawn creature tokens. They have \"Sacrifice this token: Add {C}.\"".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_spawn_tokens,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{20}: Cast any number of Eldrazi spells from among cards you own outside the game without paying their mana costs.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{20}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: cast_from_outside,
            }),
    )
}

fn make_spawn_tokens(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spawn = reg.interner().lookup("Eldrazi Spawn").unwrap_or_default();
    // GAP: token's "Sacrifice this token: Add {C}." activated ability cannot be
    // authored on a TokenDefinition via this API (abilities: vec![] only).
    let token = TokenDefinition {
        name: spawn,
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes: {
            let mut s = SubtypeSet::default();
            s.0.insert(spawn);
            s
        },
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: ctx.controller, token: token.clone() },
        Effect::CreateToken { controller: ctx.controller, token },
    ]
}

fn cast_from_outside(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "cast Eldrazi spells from outside the game" has no expressible Effect.
    Vec::new()
}
