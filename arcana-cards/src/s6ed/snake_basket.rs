//! Snake Basket — `{4}` artifact (Visions).
//! "{X}, Sacrifice this artifact: Create X 1/1 green Snake creature
//! tokens. Activate only as a sorcery."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Snake Basket");
    let _snake = reg.interner_mut().intern("Snake");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{X}, Sacrifice this artifact: Create X 1/1 green Snake creature tokens. Activate only as a sorcery.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{X}").expect("valid cost"),
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_snakes,
        }),
    )
}

fn make_snakes(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let snake = reg.interner().lookup("Snake").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake.clone());
    let x = ctx.x_value.unwrap_or(0);
    (0..x)
        .map(|_| Effect::CreateToken {
            controller: ctx.controller,
            token: arcana_core::effects::TokenDefinition {
                name: snake.clone(),
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes: subtypes.clone(),
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        })
        .collect()
}
