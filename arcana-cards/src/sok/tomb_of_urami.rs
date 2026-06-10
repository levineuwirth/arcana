//! Tomb of Urami — legendary land (Saviors of Kamigawa, 2005).
//! "{T}: Add {B}. Tomb of Urami deals 1 damage to you if you don't
//! control an Ogre." and "{2}{B}{B}, {T}, Sacrifice all lands you
//! control: Create Urami, a legendary 5/5 black Demon Spirit creature
//! token with flying."
//!
//! GAP: the conditional self-damage rider on the mana ability is not
//! expressible (a mana ability's only result may be `Effect::AddMana`).
//! GAP: "Sacrifice all lands you control" as an activation COST is not
//! expressible — approximated as sacrifice-self cost plus a resolution
//! `Effect::Sacrifice` of all your remaining lands. GAP: the token's
//! LEGENDARY supertype is not representable on `TokenDefinition`.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tomb of Urami");
    // Pre-intern the token's name and subtypes for resolve-time lookup.
    let _urami = reg.interner_mut().intern("Urami");
    let _demon = reg.interner_mut().intern("Demon");
    let _spirit = reg.interner_mut().intern("Spirit");
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
                text: "{T}: Add {B}. Tomb of Urami deals 1 damage to you \
                       if you don't control an Ogre."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                // GAP: "deals 1 damage to you if you don't control an
                // Ogre" rider — a mana ability may only AddMana.
                effect: add_black_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}{B}, {T}, Sacrifice all lands you control: \
                       Create Urami, a legendary 5/5 black Demon Spirit \
                       creature token with flying."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}{B}")
                        .expect("valid cost"),
                    tap: true,
                    // GAP: "Sacrifice all lands you control" cost —
                    // modeled as sacrifice-self; the remaining lands are
                    // sacrificed at resolution instead of as a cost.
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_urami,
            }),
    )
}

fn add_black_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)],
    }]
}

fn make_urami(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let land_filter =
        ObjectFilter { types: Some(TypeLine::LAND.into()), ..Default::default() };
    let land_count = script::count_matching(state, &land_filter, ctx.controller);
    let urami = reg.interner().lookup("Urami").unwrap_or_default();
    let mut token_subtypes = SubtypeSet::default();
    if let Some(demon) = reg.interner().lookup("Demon") {
        token_subtypes.0.insert(demon);
    }
    if let Some(spirit) = reg.interner().lookup("Spirit") {
        token_subtypes.0.insert(spirit);
    }
    vec![
        Effect::Sacrifice {
            player: ctx.controller,
            filter: land_filter,
            count: land_count,
        },
        // GAP: TokenDefinition has no supertypes field — the token's
        // LEGENDARY supertype is dropped.
        Effect::CreateToken {
            controller: ctx.controller,
            token: TokenDefinition {
                name: urami,
                colors: ColorSet::black(),
                types: TypeLine::CREATURE.into(),
                subtypes: token_subtypes,
                power: Some(PtValue::Fixed(5)),
                toughness: Some(PtValue::Fixed(5)),
                keywords: vec![KeywordAbility::Flying],
                abilities: vec![],
            },
        },
    ]
}
