//! Wall of Kelp — `{U}{U}` 0/3 Plant Wall with Defender.
//!
//! "{U}{U}, {T}: Create a 0/1 blue Plant Wall creature token with defender
//! named Kelp."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wall of Kelp");
    let plant = reg.interner_mut().intern("Plant");
    let wall = reg.interner_mut().intern("Wall");
    let _kelp = reg.interner_mut().intern("Kelp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{U}{U}, {T}: Create a 0/1 blue Plant Wall creature token with defender named Kelp.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{U}{U}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_kelp,
        }),
    )
}

fn make_kelp(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let kelp = reg.interner().lookup("Kelp").unwrap_or_default();
    let plant = reg.interner().lookup("Plant").unwrap_or_default();
    let wall = reg.interner().lookup("Wall").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(wall);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: kelp,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Defender],
            abilities: vec![],
        },
    }]
}
