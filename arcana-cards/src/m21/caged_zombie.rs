//! Caged Zombie — `{2}{B}` 2/3 black Zombie.
//! "{1}{B}, {T}: Each opponent loses 2 life. Activate only if a creature
//! died this turn."
//!
//! "Activate only if a creature died this turn" is enforced via
//! `ActivationCost::activation_condition` + `conditions::a_creature_died_this_turn`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Caged Zombie");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}, {T}: Each opponent loses 2 life. Activate only if a creature died this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").unwrap(),
                    tap: true,
                    // Only if a creature died this turn.
                    activation_condition: Some(|s, _src, _you, _reg| {
                        arcana_core::conditions::a_creature_died_this_turn(s)
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: opponents_lose_life,
            }),
    )
}

fn opponents_lose_life(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opponents = script::opponents(state, ctx.controller);
    let effects: Vec<Effect> = opponents
        .into_iter()
        .map(|p| Effect::LoseLife { player: p, amount: 2 })
        .collect();
    vec![Effect::Sequence(effects)]
}
