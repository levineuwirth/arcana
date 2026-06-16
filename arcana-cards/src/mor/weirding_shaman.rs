//! Weirding Shaman — `{1}{B}` 2/1 black Goblin Shaman. "{3}{B}, Sacrifice a
//! Goblin: Create two 1/1 black Goblin Rogue creature tokens."
//!
//! GAP: "Sacrifice a Goblin" (non-self, specific subtype) activation cost not
//! expressible via ActivationCost.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Weirding Shaman");
    let goblin = reg.interner_mut().intern("Goblin");
    let shaman = reg.interner_mut().intern("Shaman");
    let _rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{B}, Sacrifice a Goblin: Create two 1/1 black Goblin Rogue creature tokens.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{B}").unwrap(),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: create_goblin_tokens,
            }),
    )
}

fn create_goblin_tokens(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Sacrifice a Goblin" (non-self) cost not expressible.
    let goblin = reg.interner().lookup("Goblin").expect("Goblin interned during register()");
    let rogue = reg.interner().lookup("Rogue").expect("Rogue interned during register()");
    let make_token = || {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(goblin);
        subtypes.0.insert(rogue);
        TokenDefinition {
            name: goblin,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        }
    };
    vec![
        Effect::CreateToken { controller: ctx.controller, token: make_token() },
        Effect::CreateToken { controller: ctx.controller, token: make_token() },
    ]
}
