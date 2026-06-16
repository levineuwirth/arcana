//! Silhana Starfletcher — `{2}{G}` 1/3 Elf Druid Archer with Reach.
//!
//! * Reach (keyword).
//! * "As this creature enters, choose a color." — a characteristic-
//!   defining ETB CHOICE (no trigger word, stores a color on the
//!   permanent). The engine has no chosen-color storage primitive, so
//!   this is GAP'd (see below).
//! * "{T}: Add one mana of the chosen color." — the produced color
//!   depends on the (unstored) chosen color above; `Effect::AddMana`
//!   requires a concrete `ManaColor`, so the mana ability's effect is
//!   GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: "As this creature enters, choose a color." — characteristic-
// defining ETB color choice; no chosen-color storage primitive exists.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Silhana Starfletcher");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);
    subtypes.0.insert(archer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add one mana of the chosen color.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_chosen_color,
            }),
    )
}

fn add_chosen_color(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: produces mana of the color chosen as the creature entered;
    // no chosen-color storage primitive, so the color is undeterminable.
    Vec::new()
}
