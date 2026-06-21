//! Shifting Ceratops — `{2}{G}{G}` 5/4 green Dinosaur.
//!
//! Oracle:
//! * "This spell can't be countered." — a cast-time static; no expressible
//!   uncounterable primitive. GAP'd.
//! * "Protection from blue" — the `Protection` keyword has no
//!   `KeywordAbility` variant and there is no protection-from-color
//!   primitive. GAP'd.
//! * "{G}: This creature gains your choice of reach, trample, or haste
//!   until end of turn." — an activated ability whose cost ({G}) is
//!   expressible, but whose effect (a player choice between three
//!   keywords) has no modal-on-activated / choose-a-keyword primitive
//!   (modal dispatch is spell-ability only). The cost is emitted; the
//!   effect body is GAP'd (granting any single fixed keyword would be
//!   unfaithful to "your choice of").

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
    let name = reg.interner_mut().intern("Shifting Ceratops");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: keyword — "Protection from blue" (no Protection KeywordAbility
        // variant; protection-from-color is not expressible).
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static — "This spell can't be countered" (no uncounterable hook).
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}: This creature gains your choice of reach, trample, or haste until end of turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: choose_keyword,
            }),
    )
}

fn choose_keyword(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "gains your choice of reach, trample, or haste" — no choose-a-
    // keyword primitive (modal dispatch is spell-ability only); granting a
    // fixed keyword would be unfaithful to the printed player choice.
    Vec::new()
}
