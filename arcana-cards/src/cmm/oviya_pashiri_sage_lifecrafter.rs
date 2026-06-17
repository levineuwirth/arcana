//! Oviya Pashiri, Sage Lifecrafter — `{G}` 1/2 Legendary Human Artificer.
//! "{2}{G}, {T}: Create a 1/1 colorless Servo artifact creature token."
//! "{4}{G}, {T}: Create an X/X colorless Construct artifact creature token,
//!  where X is the number of creatures you control."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oviya Pashiri, Sage Lifecrafter");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);
    // Pre-intern token subtypes.
    let _servo = reg.interner_mut().intern("Servo");
    let _construct = reg.interner_mut().intern("Construct");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G}, {T}: Create a 1/1 colorless Servo artifact creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_servo,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{G}, {T}: Create an X/X colorless Construct artifact creature token, where X is the number of creatures you control.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{G}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_construct,
            }),
    )
}

fn make_servo(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let Some(servo) = reg.interner().lookup("Servo") else {
        return Vec::new();
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(servo);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: servo,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn make_construct(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let Some(construct) = reg.interner().lookup("Construct") else {
        return Vec::new();
    };
    let x = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    ) as i32;
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: construct,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(x)),
            toughness: Some(PtValue::Fixed(x)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
