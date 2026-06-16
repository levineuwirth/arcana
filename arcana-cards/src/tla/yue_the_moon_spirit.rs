//! Yue, the Moon Spirit — `{3}{U}` 3/3 Legendary Spirit Ally with
//! Flying and Vigilance.
//!
//! Oracle:
//! * Flying, vigilance.
//! * Waterbend {5}, {T}: You may cast a noncreature spell from your hand
//!   without paying its mana cost.
//!
//! Waterbend is not a usable `KeywordAbility`; its mana+tap cost is modeled,
//! but the "cast a spell from your hand without paying its mana cost" payload
//! is not expressible with the demonstrated Effect surface — GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yue, the Moon Spirit");
    let spirit = reg.interner_mut().intern("Spirit");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Waterbend {5}, {T}: You may cast a noncreature spell from your hand without paying its mana cost.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{5}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: waterbend_free_cast,
        }),
    )
}

fn waterbend_free_cast(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "cast a noncreature spell from your hand without paying its mana cost"
    // is not expressible with the demonstrated Effect surface.
    Vec::new()
}
