//! Meria, Scholar of Antiquity — `{1}{R}{G}` 3/3 Legendary Elf Artificer.
//! - Tap an untapped nontoken artifact you control: Add {G}.
//! - Tap two untapped nontoken artifacts you control: Exile the top card
//!   of your library. You may play it this turn.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Meria, Scholar of Antiquity");
    let elf = reg.interner_mut().intern("Elf");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap an untapped nontoken artifact you control: Add {G}.".into(),
                cost: ActivationCost {
                    tap_other: Some(
                        ObjectFilter::permanent()
                            .with_types(TypeLine::ARTIFACT.into())
                            .nontoken(),
                    ),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_green,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap two untapped nontoken artifacts you control: Exile the top card of your library. You may play it this turn.".into(),
                cost: ActivationCost {
                    tap_other: Some(
                        ObjectFilter::permanent()
                            .with_types(TypeLine::ARTIFACT.into())
                            .nontoken(),
                    ),
                    tap_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: impulse_one,
            }),
    )
}

/// Tap an untapped nontoken artifact you control: Add {G}.
fn add_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}

/// Tap two ...: exile the top card of your library; you may play it this
/// turn.
fn impulse_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ImpulseExile {
        player: ctx.controller,
        count: 1,
    }]
}
