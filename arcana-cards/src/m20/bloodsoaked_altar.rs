//! Bloodsoaked Altar — `{4}{B}{B}` artifact.
//! "{T}, Pay 2 life, Discard a card, Sacrifice a creature: Create a 5/5
//! black Demon creature token with flying. Activate only as a sorcery."
//! The full compound cost is expressible on one `ActivationCost` (tap +
//! life + discard_other + sacrifice_other); the effect mints the Demon
//! token.

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
    let name = reg.interner_mut().intern("Bloodsoaked Altar");
    let _demon = reg.interner_mut().intern("Demon");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}, Pay 2 life, Discard a card, Sacrifice a \
                       creature: Create a 5/5 black Demon creature token \
                       with flying. Activate only as a sorcery."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    life: 2,
                    discard_other: Some(ObjectFilter::default()),
                    sacrifice_other: Some(ObjectFilter::creature()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_demon,
            },
        ),
    )
}

fn make_demon(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let demon = reg.interner().lookup("Demon").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon.clone());
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: demon,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
