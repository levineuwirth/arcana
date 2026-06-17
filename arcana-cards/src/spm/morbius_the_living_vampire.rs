//! Morbius the Living Vampire — `{2}{U}{B}` 3/1 legendary Vampire
//! Scientist Villain with Flying, Vigilance, Lifelink.
//! "{U}{B}, Exile this card from your graveyard: Look at the top three
//! cards of your library. Put one of them into your hand and the rest
//! on the bottom of your library in any order."

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Morbius the Living Vampire");
    let vampire = reg.interner_mut().intern("Vampire");
    let scientist = reg.interner_mut().intern("Scientist");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(scientist);
    subtypes.0.insert(villain);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Vigilance,
            KeywordAbility::Lifelink,
        ],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{U}{B}, Exile this card from your graveyard: Look at the top three cards of your library. Put one of them into your hand and the rest on the bottom of your library in any order.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{U}{B}").expect("valid cost"),
                exile_self: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: dig_three,
        }),
    )
}

fn dig_three(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 3,
        filter: None,
        rest: DigRest::BottomRandom,
    }]
}
