//! Illustrious Historian — `{1}{R}` 2/1 red Creature — Human Shaman.
//! {5}, Exile this card from your graveyard: Create a tapped 3/2 red and white Spirit creature token.
//! This is a graveyard activated ability (activation_zone: Graveyard).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Illustrious Historian");
    let human_sub = reg.interner_mut().intern("Human");
    let shaman_sub = reg.interner_mut().intern("Shaman");
    let _spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(shaman_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}, Exile this card from your graveyard: Create a tapped 3/2 red and white Spirit creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}").unwrap(),
                    exile_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: create_spirit,
            }),
    )
}

fn create_spirit(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spirit = reg.interner().lookup("Spirit").expect("interned at register");
    let mut ts = SubtypeSet::default();
    ts.0.insert(spirit);
    let token = TokenDefinition {
        name: spirit,
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: ts,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    // Creating tapped token — CreateToken creates untapped; tapped variant not available, using plain CreateToken
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}
