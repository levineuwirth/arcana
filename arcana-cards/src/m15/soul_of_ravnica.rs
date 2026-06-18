//! Soul of Ravnica — `{4}{U}{U}` 6/6 Avatar.
//! Flying.
//! {5}{U}{U}: Draw a card for each color among permanents you control.
//! {5}{U}{U}, Exile this card from your graveyard: Draw a card for each color among
//! permanents you control.
//! Both abilities are wired structurally, but "for each color among permanents you
//! control" has no script:: helper to count distinct colors — the dynamic amount is
//! GAP'd (resolvers return no effects rather than hardcoding a literal).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soul of Ravnica");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{U}{U}: Draw a card for each color among permanents you control.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{U}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_per_color,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{U}{U}, Exile this card from your graveyard: Draw a card for each color among permanents you control.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{U}{U}").expect("valid cost"),
                    exile_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_per_color,
            }),
    )
}

fn draw_per_color(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "draw a card for each color among permanents you control" — no script::
    // helper counts distinct colors among permanents; dynamic amount not computable.
    Vec::new()
}
