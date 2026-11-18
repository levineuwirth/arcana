//! Port Town — nonbasic land (Shadows over Innistrad, 2016).
//! "As this land enters, you may reveal a Plains or Island card from
//! your hand. If you don't, this land enters tapped." and
//! "{T}: Add {W} or {U}."
//! The reveal is an entry decision (`EntersWithSpec::TappedUnlessReveal`):
//! its controller reveals a Plains or Island card from hand as it enters,
//! or it enters tapped.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Port Town");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    // The land subtypes are interned here so the entry decision needs no
    // registry: a card in hand matches by its own subtypes, so a dual
    // land with either type can be revealed. The filter asks for a land
    // card: these are land types, which a changeling's "every creature
    // type" does not include (CR 205.3, 702.73a).
    let plains = reg.interner_mut().intern("Plains");
    let island = reg.interner_mut().intern("Island");
    let reveal = EntersWithSpec::TappedUnlessReveal {
        filter: ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .with_subtypes_any(vec![plains, island]),
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(reveal)
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
                text: "{T}: Add {U}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_blue_mana,
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

fn add_blue_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
    }]
}
