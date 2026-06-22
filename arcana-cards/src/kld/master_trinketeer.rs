//! Master Trinketeer — `{2}{W}` 3/2 Creature — Dwarf Artificer.
//! "Servos and Thopters you control get +1/+1."
//! "{3}{W}: Create a 1/1 colorless Servo artifact creature token."
//!
//! GAP (static): "Servos and Thopters you control get +1/+1" is a continuous
//! anthem static — not a triggered/activated ability and no demonstrated Effect
//! expresses a filtered continuous buff.
//! The {3}{W} activated ability creates a 1/1 colorless Servo artifact creature
//! token.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Master Trinketeer");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let artificer = reg.interner_mut().intern("Artificer");
    let _servo = reg.interner_mut().intern("Servo");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{W}: Create a 1/1 colorless Servo artifact creature token.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{W}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_servo,
        }),
    )
}

fn make_servo(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let servo = reg.interner().lookup("Servo").unwrap_or_default();
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
