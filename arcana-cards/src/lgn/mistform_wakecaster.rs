//! Mistform Wakecaster — `{4}{U}` 2/3 Creature — Illusion.
//! Flying.
//! {1}: This creature becomes the creature type of your choice until end
//!   of turn.
//! {2}{U}{U}, {T}: Choose a creature type. Each creature you control
//!   becomes that type until end of turn.
//!
//! GAP: there is no Effect for "becomes a creature type of your choice"
//! (a chosen-subtype set, not the additive AddType primitive). Both
//! activated abilities are emitted with their costs but empty effects.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mistform Wakecaster");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}: This creature becomes the creature type of your choice until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_chosen_type,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}{U}, {T}: Choose a creature type. Each creature you control becomes that type until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}{U}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: all_become_chosen_type,
            }),
    )
}

fn become_chosen_type(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no Effect to set a creature's subtype to a chosen creature type.
    Vec::new()
}

fn all_become_chosen_type(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no Effect to set creatures' subtypes to a chosen creature type.
    Vec::new()
}
