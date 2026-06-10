//! The Hive — `{5}` artifact.
//! "{5}, {T}: Create a 1/1 colorless Insect artifact creature token with
//! flying named Wasp."

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
    let name = reg.interner_mut().intern("The Hive");
    let _wasp = reg.interner_mut().intern("Wasp");
    let _insect = reg.interner_mut().intern("Insect");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{5}, {T}: Create a 1/1 colorless Insect artifact creature token with flying named Wasp.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{5}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_wasp,
        }),
    )
}

fn make_wasp(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wasp = reg.interner().lookup("Wasp").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    if let Some(insect) = reg.interner().lookup("Insect") {
        subtypes.0.insert(insect);
    }
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: wasp,
            colors: ColorSet::new(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
