//! Bant Panorama — nonbasic land.
//! "{T}: Add {C}." and "{1}, {T}, Sacrifice this land: Search your
//! library for a basic Forest, Plains, or Island card, put it onto the
//! battlefield tapped, then shuffle."
//! The fetch is `Effect::TutorToBattlefield` (tapped) over a
//! basic-supertype land filter with a Forest/Plains/Island subtype OR.

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
    let name = reg.interner_mut().intern("Bant Panorama");
    let _forest = reg.interner_mut().intern("Forest");
    let _plains = reg.interner_mut().intern("Plains");
    let _island = reg.interner_mut().intern("Island");
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
                text: "{1}, {T}, Sacrifice this land: Search your library for a basic Forest, Plains, or Island card, put it onto the battlefield tapped, then shuffle.".into(),
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
    let mut subs = Vec::new();
    for n in ["Forest", "Plains", "Island"] {
        if let Some(s) = reg.interner().lookup(n) {
            subs.push(s);
        }
    }
    let filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC))
        .with_subtypes_any(subs);
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter,
        tapped: true,
    }]
}
