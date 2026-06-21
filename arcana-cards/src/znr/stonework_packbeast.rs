//! Stonework Packbeast — `{2}` 2/1 Artifact Creature — Beast.
//!
//! * "Stonework Packbeast is also a Cleric, Rogue, Warrior, and Wizard."
//!   This unconditional, permanent type-grant is folded directly into
//!   the base subtype set (Beast + Cleric + Rogue + Warrior + Wizard) —
//!   a faithful simplification of the static, since nothing gates it.
//! * "{2}: Add one mana of any color." An activated ability with a {2}
//!   cost; the "any color" payload has no choose-a-color mana primitive
//!   in the demonstrated surface, so the effect body is GAP'd (emitting
//!   a fixed color would be materially wrong).

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
    let name = reg.interner_mut().intern("Stonework Packbeast");
    let beast = reg.interner_mut().intern("Beast");
    let cleric = reg.interner_mut().intern("Cleric");
    let rogue = reg.interner_mut().intern("Rogue");
    let warrior = reg.interner_mut().intern("Warrior");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    subtypes.0.insert(cleric);
    subtypes.0.insert(rogue);
    subtypes.0.insert(warrior);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}: Add one mana of any color.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                ..ActivationCost::default()
            },
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
    // GAP: "Add one mana of any color" — no choose-a-color mana
    // primitive is in scope; emitting a fixed color would be wrong.
    Vec::new()
}
