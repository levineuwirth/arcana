//! Birthing Boughs — `{3}` artifact (Modern Horizons, 2019).
//! "{4}, {T}: Create a 2/2 colorless Shapeshifter creature token with
//! changeling." One token-minting activated ability; the token carries
//! `KeywordAbility::Changeling`.

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
    let name = reg.interner_mut().intern("Birthing Boughs");
    let _shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{4}, {T}: Create a 2/2 colorless Shapeshifter \
                       creature token with changeling."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_shapeshifter,
            },
        ),
    )
}

fn make_shapeshifter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let shapeshifter = reg.interner().lookup("Shapeshifter").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: shapeshifter,
            colors: ColorSet::new(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Changeling],
            abilities: vec![],
        },
    }]
}
