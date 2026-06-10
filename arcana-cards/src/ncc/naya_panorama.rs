//! Naya Panorama — nonbasic land (panorama fetch).
//! "{T}: Add {C}." and "{1}, {T}, Sacrifice this land: Search your
//! library for a basic Mountain, Forest, or Plains card, put it onto
//! the battlefield tapped, then shuffle."

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, ManaColor, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Naya Panorama");
    // Pre-intern the basic land subtypes for the resolver's read-only
    // lookup.
    let _mountain = reg.interner_mut().intern("Mountain");
    let _forest = reg.interner_mut().intern("Forest");
    let _plains = reg.interner_mut().intern("Plains");
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
                text: "{1}, {T}, Sacrifice this land: Search your library \
                       for a basic Mountain, Forest, or Plains card, put it \
                       onto the battlefield tapped, then shuffle."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
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
                effect: fetch_basic,
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

fn fetch_basic(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut syms = Vec::new();
    if let Some(s) = reg.interner().lookup("Mountain") {
        syms.push(s);
    }
    if let Some(s) = reg.interner().lookup("Forest") {
        syms.push(s);
    }
    if let Some(s) = reg.interner().lookup("Plains") {
        syms.push(s);
    }
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC))
            .with_subtypes_any(syms),
        tapped: true,
    }]
}
