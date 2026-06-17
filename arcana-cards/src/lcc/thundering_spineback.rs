//! Thundering Spineback — `{5}{G}{G}` 5/5 Dinosaur.
//! "Other Dinosaurs you control get +1/+1." (static anthem — GAP)
//! "{5}{G}: Create a 3/3 green Dinosaur creature token with trample."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thundering Spineback");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    // GAP: "Other Dinosaurs you control get +1/+1." — static anthem, not a
    // triggered/activated ability.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{5}{G}: Create a 3/3 green Dinosaur creature token with trample.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{5}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_dino_token,
        }),
    )
}

fn make_dino_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dino = reg
        .interner()
        .lookup("Dinosaur")
        .unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dino);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: dino,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Trample],
            abilities: vec![],
        },
    }]
}
