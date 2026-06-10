//! Grove of the Guardian — nonbasic land.
//! "{T}: Add {C}." and "{3}{G}{W}, {T}, Tap two untapped creatures you
//! control, Sacrifice this land: Create an 8/8 green and white Elemental
//! creature token with vigilance." The mana + tap + sacrifice-self cost
//! components are wired; the 'tap two untapped creatures you control'
//! component is a documented cost gap.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grove of the Guardian");
    let _elemental = reg.interner_mut().intern("Elemental");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{G}{W}, {T}, Tap two untapped creatures you \
                       control, Sacrifice this land: Create an 8/8 green \
                       and white Elemental creature token with vigilance."
                    .into(),
                // GAP: cost component 'Tap two untapped creatures you
                // control' — ActivationCost has no tap-other-permanents
                // field; the remaining cost is wired as printed.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{G}{W}")
                        .expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_guardian,
            }),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn make_guardian(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elemental = reg.interner().lookup("Elemental").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental.clone());
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: elemental,
            colors: ColorSet::green() | ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(8)),
            toughness: Some(PtValue::Fixed(8)),
            keywords: vec![KeywordAbility::Vigilance],
            abilities: vec![],
        },
    }]
}
