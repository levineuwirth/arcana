//! Carnival Barker — `{2}{R}` 3/3 red Dog Employee.
//! "{T}: You have thirty seconds to laud a creature you control to any people outside
//! the game. Until end of turn, that creature gains trample and haste and gets +X/+0,
//! where X is the number of those people who applauded."
//! GAP: "people outside the game" and "count applauders" — real-world social mechanic
//! not expressible in the engine.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Carnival Barker");
    let dog = reg.interner_mut().intern("Dog");
    let employee = reg.interner_mut().intern("Employee");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    subtypes.0.insert(employee);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: You have thirty seconds to laud a creature. Creature gains trample and haste and gets +X/+0 where X = applauders.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: laud_creature,
            }),
    )
}

fn laud_creature(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "thirty seconds to laud, count applauders outside the game" — real-world
    // social mechanic; no equivalent in engine catalog.
    Vec::new()
}
