//! Socketed Sprocketer — `{U}` 1/1 Artifact Creature — Cyborg Knight.
//! {T}: Uninstall all results, then roll a six-sided die and install the
//! result on this creature.
//! You may uninstall a result to use it for a die you rolled.
//! Uninstall a 6 from this creature: Draw a card.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Socketed Sprocketer");
    let cyborg = reg.interner_mut().intern("Cyborg");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cyborg);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: static permission "You may uninstall a result to use it for a die
    // you rolled" — no install/die mechanic.
    // GAP: "Uninstall a 6 from this creature: Draw a card" — uninstall is not
    // an expressible activation cost; the whole ability is omitted.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Uninstall all results from this creature, then roll a six-sided die. Install the result on this creature.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: roll_and_install,
        }),
    )
}

fn roll_and_install(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "uninstall all results, roll a d6, install the result" — no die /
    // install-result mechanic in the engine.
    Vec::new()
}
