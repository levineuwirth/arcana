//! Reclusive Taxidermist — `{1}{G}` 1/2 Creature — Human Druid.
//! "This creature gets +3/+2 as long as there are four or more creature cards
//! in your graveyard." "{T}: Add one mana of any color."
//!
//! The conditional static buff is GAP'd (no conditional continuous-buff
//! primitive in this card class). The tap mana ability's cost is expressible,
//! but "add one mana of any color" has no choose-a-color primitive, so its
//! effect body GAPs (a fixed color would be materially wrong).

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
    let name = reg.interner_mut().intern("Reclusive Taxidermist");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    // GAP: "gets +3/+2 as long as there are four or more creature cards in your
    // graveyard" — conditional static continuous buff, not expressible here.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Add one mana of any color.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_any_color,
        }),
    )
}

fn add_any_color(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Add one mana of any color" — no choose-a-color mana primitive is
    // available; emitting a fixed color would be materially wrong.
    Vec::new()
}
