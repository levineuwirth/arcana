//! Walking Desecration — `{2}{B}` 1/1 black Zombie.
//! "{B}, {T}: Creatures of the creature type of your choice attack this turn if able."
//! GAP: the engine has `filtered_must_attack` for a FIXED filter, but there is
//! no runtime "choose a creature type" selection to build that filter from at
//! activation time — so the chosen-subtype board-wide must-attack can't be
//! expressed faithfully.

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
    let name = reg.interner_mut().intern("Walking Desecration");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}, {T}: Creatures of the creature type of your choice attack this turn if able.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: force_type_attack,
            }),
    )
}

fn force_type_attack(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the type-wide forced attack is expressible (filtered_must_attack),
    // but choosing "a creature type" at activation time to build that filter
    // is not — no runtime creature-type selection primitive.
    Vec::new()
}
