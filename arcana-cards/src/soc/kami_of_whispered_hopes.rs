//! Kami of Whispered Hopes — `{2}{G}` 1/1 Spirit.
//! Replacement: +1/+1 counters put on permanents you control get +1 extra.
//! `{T}: Add X mana of any one color, where X is this creature's power.`
//!
//! Both lines are GAP'd: the counter-doubling-style replacement is a static
//! replacement effect with no triggered/activated form, and the mana ability
//! produces "any one color" (no choose-a-color mana primitive).

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
    let name = reg.interner_mut().intern("Kami of Whispered Hopes");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "If one or more +1/+1 counters would be put on a permanent you
    // control, that many plus one are put instead" — a static counter-adding
    // replacement effect with no decomposable trigger/activation.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Add X mana of any one color, where X is this creature's power.".into(),
            cost: ActivationCost { tap: true, ..ActivationCost::default() },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_x_any_one_color,
        }),
    )
}

fn add_x_any_one_color(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Add X mana of any one color" — no choose-a-color mana primitive;
    // emitting a fixed color would be materially wrong.
    Vec::new()
}
