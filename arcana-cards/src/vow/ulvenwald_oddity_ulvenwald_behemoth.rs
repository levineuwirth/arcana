//! Ulvenwald Oddity // Ulvenwald Behemoth
//!
//! Front (Ulvenwald Oddity, {2}{G}{G}, 4/4 Beast):
//!   Trample, haste
//!   {5}{G}{G}: Transform this creature. (activated ability, front-face only)
//!
//! Back (Ulvenwald Behemoth, Beast Horror):
//!   Trample, haste
//!   Other creatures you control get +1/+1 and have trample and haste.
//!   (GAP: back-face-only static anthem not modeled.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ulvenwald Oddity");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ulvenwald Behemoth");
    let beast2 = reg.interner_mut().intern("Beast");
    let horror = reg.interner_mut().intern("Horror");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(beast2);
    back_subtypes.0.insert(horror);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(8)),
            toughness: Some(PtValue::Fixed(8)),
            keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {5}{G}{G}: Transform — front face only
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{G}{G}: Transform this creature.".to_string(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{G}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: transform_self,
            })
            // GAP: back-face-only static anthem (other creatures +1/+1, trample, haste) not modeled.
    )
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
