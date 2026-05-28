//! Krenko, Mob Boss — `{2}{R}{R}` 3/3 Legendary red Goblin Warrior. "{T}: Create X
//! 1/1 red Goblin creature tokens, where X is the number of Goblins you control."

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Krenko, Mob Boss");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Create X 1/1 red Goblin creature tokens, where X is the number of Goblins you control.".into(),
                cost: ActivationCost::tap_only(),
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
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let goblin_filter = script::subtype_filter(reg, "Goblin");
    let x = script::count_matching(state, &goblin_filter, ctx.controller) as usize;
    let goblin_id = reg.interner().lookup("Goblin")
        .expect("Goblin interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(goblin_id);
    (0..x).map(|_| {
        Effect::CreateToken {
            controller: ctx.controller,
            token: TokenDefinition {
                name: goblin_id,
                colors: ColorSet::red(),
                types: TypeLine::CREATURE.into(),
                subtypes: token_subtypes.clone(),
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        }
    }).collect()
}
