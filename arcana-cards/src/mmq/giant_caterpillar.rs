//! Giant Caterpillar — `{3}{G}` 3/3 green Insect. "{G}, Sacrifice this creature:
//! Create a 1/1 green Insect creature token with flying named Butterfly at
//! the beginning of the next end step."
//!
//! GAP: "at the beginning of the next end step" — creating immediately as
//! approximation.

use arcana_core::effects::{Effect, TokenDefinition, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Giant Caterpillar");
    let insect = reg.interner_mut().intern("Insect");
    let _butterfly = reg.interner_mut().intern("Butterfly");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}, Sacrifice this creature: Create a 1/1 green Insect creature token with flying named Butterfly at the beginning of the next end step.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}").unwrap(),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: create_butterfly,
            }),
    )
}

fn create_butterfly(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "at the beginning of the next end step" delay not modeled — creating immediately.
    let butterfly = reg.interner().lookup("Butterfly").expect("Butterfly interned during register()");
    let insect = reg.interner().lookup("Insect").expect("Insect interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    let token = TokenDefinition {
        name: butterfly,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}
