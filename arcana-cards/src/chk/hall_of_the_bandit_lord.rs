//! Hall of the Bandit Lord — Legendary Land (Champions of Kamigawa).
//! "Hall of the Bandit Lord enters tapped." and "{T}, Pay 3 life:
//! Add {C}. If that mana is spent on a creature spell, it gains
//! haste."
//!
//! GAP: the mana-spend rider ("If that mana is spent on a creature
//! spell, it gains haste") is not expressible — mana spend tracking /
//! riders are unmodeled. The plain pay-3-life colorless mana ability
//! is wired.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hall of the Bandit Lord");
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
            .with_enters_with(EntersWithSpec::Tapped)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Pay 3 life: Add {C}. If that mana is spent on \
                       a creature spell, it gains haste."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    life: 3,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            }),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If that mana is spent on a creature spell, it gains haste"
    // — mana spend-rider not expressible.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}
