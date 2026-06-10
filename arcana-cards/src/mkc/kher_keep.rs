//! Kher Keep — legendary nonbasic land.
//! "{T}: Add {C}." and "{1}{R}, {T}: Create a 0/1 red Kobold creature
//! token named Kobolds of Kher Keep."
//! A colorless mana ability plus a token-minting activation.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kher Keep");
    let _token_name = reg.interner_mut().intern("Kobolds of Kher Keep");
    let _kobold = reg.interner_mut().intern("Kobold");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
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
                text: "{1}{R}, {T}: Create a 0/1 red Kobold creature token named Kobolds of Kher Keep.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_kobold,
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

fn make_kobold(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let nm = reg.interner().lookup("Kobolds of Kher Keep").unwrap_or_default();
    let kobold = reg.interner().lookup("Kobold").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kobold);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: nm,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
