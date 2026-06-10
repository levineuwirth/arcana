//! Fire-Lit Thicket — nonbasic land (Shadowmoor, 2008).
//! "{T}: Add {C}." and "{R/G}, {T}: Add {R}{R}, {R}{G}, or {G}{G}."
//! The three-way filter choice is modeled as three separately
//! activatable mana abilities (the catalog-wide "Add X or Y" idiom).

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fire-Lit Thicket");
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
                text: "{R/G}, {T}: Add {R}{R}.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R/G}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_rr,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R/G}, {T}: Add {R}{G}.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R/G}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_rg,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R/G}, {T}: Add {G}{G}.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R/G}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_gg,
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

fn add_rr(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Red, ctx.source),
            ManaUnit::plain(ManaColor::Red, ctx.source),
        ],
    }]
}

fn add_rg(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Red, ctx.source),
            ManaUnit::plain(ManaColor::Green, ctx.source),
        ],
    }]
}

fn add_gg(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Green, ctx.source),
            ManaUnit::plain(ManaColor::Green, ctx.source),
        ],
    }]
}
