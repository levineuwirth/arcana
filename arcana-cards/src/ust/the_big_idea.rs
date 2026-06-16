//! The Big Idea — `{4}{R}{R}` 4/4 Legendary Brainiac Villain.
//! {2}{B/R}{B/R}, {T}: Roll a six-sided die. Create a number of 1/1 red
//! Brainiac creature tokens equal to the result.
//! Tap three untapped Brainiacs you control: The next time you would roll a
//! six-sided die, instead roll two six-sided dice and use the total.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Big Idea");
    let brainiac = reg.interner_mut().intern("Brainiac");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(brainiac);
    subtypes.0.insert(villain);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    let brainiac_filter = script::subtype_filter(reg, "Brainiac");

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B/R}{B/R}, {T}: Roll a six-sided die. Create a number of 1/1 red Brainiac creature tokens equal to the result.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B/R}{B/R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: roll_make_tokens,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap three untapped Brainiacs you control: The next time you would roll a six-sided die, instead roll two six-sided dice and use the total.".into(),
                cost: ActivationCost {
                    tap_other: Some(brainiac_filter),
                    tap_other_count: 3,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: roll_modifier,
            }),
    )
}

fn roll_make_tokens(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: rolling a six-sided die and creating a variable number of tokens
    // equal to the result is not an expressible Effect (no die-roll primitive).
    Vec::new()
}

fn roll_modifier(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: modifying the next d6 roll into 2d6 is not an expressible Effect.
    Vec::new()
}
