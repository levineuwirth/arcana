//! Hornet Cannon — `{4}` artifact.
//! "{3}, {T}: Create a 1/1 colorless Insect artifact creature token
//! with flying and haste named Hornet. Destroy it at the beginning of
//! the next end step." The create-then-destroy-at-end-step pattern is
//! `Effect::CreateTokenSacEot`.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hornet Cannon");
    // Pre-intern the token name and subtype for the resolver's read-only
    // lookup.
    let _hornet = reg.interner_mut().intern("Hornet");
    let _insect = reg.interner_mut().intern("Insect");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{3}, {T}: Create a 1/1 colorless Insect artifact \
                       creature token with flying and haste named Hornet. \
                       Destroy it at the beginning of the next end step."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_hornet,
            },
        ),
    )
}

fn make_hornet(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let hornet = reg.interner().lookup("Hornet").unwrap_or_default();
    let insect = reg.interner().lookup("Insect").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    vec![Effect::CreateTokenSacEot {
        controller: ctx.controller,
        token: TokenDefinition {
            name: hornet,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
            abilities: vec![],
        },
    }]
}
