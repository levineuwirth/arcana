//! Raul, Trouble Shooter — `{1}{U}{B}` 1/4 Legendary Zombie Mutant Rogue.
//! Once during each of your turns, you may cast a spell from among cards in
//! your graveyard that were milled this turn. (Static — GAP'd.)
//! {T}: Each player mills a card.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Raul, Trouble Shooter");
    let zombie = reg.interner_mut().intern("Zombie");
    let mutant = reg.interner_mut().intern("Mutant");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(mutant);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "Once during each of your turns, you may cast a spell from
            // among cards in your graveyard milled this turn" — a static
            // cast-permission ability is not expressible.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Each player mills a card.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: each_player_mills,
            }),
    )
}

fn each_player_mills(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let effs = script::all_players(state)
        .into_iter()
        .map(|p| Effect::Mill { player: p, count: 1 })
        .collect();
    let _ = ctx.source;
    vec![Effect::Sequence(effs)]
}
