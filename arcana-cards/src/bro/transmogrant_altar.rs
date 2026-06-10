//! Transmogrant Altar — `{3}` artifact.
//! "{B}, {T}, Sacrifice a creature: Add {C}{C}{C}." and "{2}, {T},
//! Sacrifice a creature: Create a 3/3 colorless Zombie artifact
//! creature token. Activate only as a sorcery." Both activations pay
//! a chosen-creature sacrifice via `sacrifice_other`.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Transmogrant Altar");
    // Pre-intern the token subtype for the resolver's read-only lookup.
    let _zombie = reg.interner_mut().intern("Zombie");
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
                text: "{B}, {T}, Sacrifice a creature: Add {C}{C}{C}.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                    tap: true,
                    sacrifice_other: Some(ObjectFilter::creature()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_three_colorless,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}, Sacrifice a creature: Create a 3/3 \
                       colorless Zombie artifact creature token. Activate \
                       only as a sorcery."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    sacrifice_other: Some(ObjectFilter::creature()),
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

fn add_three_colorless(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source); 3],
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
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
