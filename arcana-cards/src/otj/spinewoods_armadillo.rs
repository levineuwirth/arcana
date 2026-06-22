//! Spinewoods Armadillo — `{4}{G}{G}` 7/7 Armadillo.
//! "Reach
//!  Ward {3}
//!  {1}{G}, Discard this card: Search your library for a basic land card
//!  or a Desert card, reveal it, put it into your hand, then shuffle. You
//!  gain 3 life."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spinewoods Armadillo");
    let armadillo = reg.interner_mut().intern("Armadillo");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(armadillo);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![
            KeywordAbility::Reach,
            KeywordAbility::Ward(ManaCost::parse("{3}").expect("valid cost")),
        ],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{G}, Discard this card: Search your library for a basic land card or a Desert card, reveal it, put it into your hand, then shuffle. You gain 3 life.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{G}").expect("valid cost"),
                discard_self: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Hand,
            is_instant_speed: false,
            face_gate: None,
            effect: search_and_gain,
        }),
    )
}

fn search_and_gain(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the exact "basic land OR Desert card" union can't be expressed
    //      in one ObjectFilter (supertype-or-subtype). We tutor a basic
    //      land card; the Desert-only branch is not represented.
    let basic_land = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC));
    vec![
        Effect::TutorToHand {
            player: ctx.controller,
            filter: basic_land,
            reveal: true,
        },
        Effect::GainLife {
            player: ctx.controller,
            amount: 3,
        },
    ]
}
