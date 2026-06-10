//! Cradle of the Accursed — nonbasic land, subtype Desert.
//! "{T}: Add {C}." and "{3}, {T}, Sacrifice this land: Create a 2/2
//! black Zombie creature token. Activate only as a sorcery." A
//! colorless mana ability plus a sacrifice-for-token utility
//! activation (sorcery speed).

use arcana_core::effects::Effect;
use arcana_core::effects::TokenDefinition;
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
    let name = reg.interner_mut().intern("Cradle of the Accursed");
    let desert = reg.interner_mut().intern("Desert");
    let _zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(desert);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
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
                text: "{3}, {T}, Sacrifice this land: Create a 2/2 black \
                       Zombie creature token. Activate only as a sorcery."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
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
                effect: make_zombie,
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

fn make_zombie(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: zombie,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
