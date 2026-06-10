//! Thopter Foundry — `{W/B}{U}` artifact (Alara Reborn, 2009).
//! "{1}, Sacrifice a nontoken artifact: Create a 1/1 blue Thopter
//! artifact creature token with flying. You gain 1 life."
//! One activated ability: mana + sacrifice-a-nontoken-artifact cost,
//! minting a flying Thopter and gaining 1 life.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thopter Foundry");
    let _ = reg.interner_mut().intern("Thopter");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W/B}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{1}, Sacrifice a nontoken artifact: Create a 1/1 blue Thopter artifact creature token with flying. You gain 1 life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    sacrifice_other: Some(
                        ObjectFilter::new()
                            .with_types(TypeLine::ARTIFACT.into())
                            .nontoken(),
                    ),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_thopter,
            },
        ),
    )
}

fn make_thopter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let thopter = reg.interner().lookup("Thopter").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(thopter);
    vec![
        Effect::CreateToken {
            controller: ctx.controller,
            token: TokenDefinition {
                name: thopter,
                colors: ColorSet::blue(),
                types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![KeywordAbility::Flying],
                abilities: vec![],
            },
        },
        Effect::GainLife { player: ctx.controller, amount: 1 },
    ]
}
