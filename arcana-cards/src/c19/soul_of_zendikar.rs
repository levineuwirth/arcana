//! Soul of Zendikar — `{4}{G}{G}` 6/6 Avatar.
//!
//! * Reach.
//! * `{3}{G}{G}: Create a 3/3 green Beast creature token.`
//! * `{3}{G}{G}, Exile this card from your graveyard: Create a 3/3 green
//!   Beast creature token.` (graveyard-activated).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soul of Zendikar");
    let avatar = reg.interner_mut().intern("Avatar");
    let _beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{G}{G}: Create a 3/3 green Beast creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{G}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_beast,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{G}{G}, Exile this card from your graveyard: Create a 3/3 green Beast creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{G}{G}").expect("valid cost"),
                    exile_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: make_beast,
            }),
    )
}

fn make_beast(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let beast = match reg.interner().lookup("Beast") {
        Some(b) => b,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let token = TokenDefinition {
        name: beast,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token,
    }]
}
