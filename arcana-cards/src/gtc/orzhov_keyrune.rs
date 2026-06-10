//! Orzhov Keyrune — `{3}` artifact.
//! "{T}: Add {W} or {B}." and "{W}{B}: This artifact becomes a 1/4
//! white and black Thrull artifact creature with lifelink until end
//! of turn." Two mana abilities (the color choice) plus an animation
//! activation (AddType + SetBasePT + SetColor + GrantKeyword; the
//! Thrull subtype gain is a GAP).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Orzhov Keyrune");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {W}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_white_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {B}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_black_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}{B}: This artifact becomes a 1/4 white and black \
                       Thrull artifact creature with lifelink until end of \
                       turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: animate,
            }),
    )
}

fn add_white_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::White, ctx.source)],
    }]
}

fn add_black_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)],
    }]
}

fn animate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the Thrull subtype gain is not expressible (AddType covers
    // card types only).
    vec![
        Effect::AddType {
            target: ctx.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: 1,
            toughness: 4,
            duration: Duration::EndOfTurn,
        },
        Effect::SetColor {
            target: ctx.source,
            colors: ColorSet::white() | ColorSet::black(),
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Lifelink,
            duration: Duration::EndOfTurn,
        },
    ]
}
