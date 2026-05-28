//! Giant Slug — `{1}{B}` 1/1 Slug.
//! `{5}: At the beginning of your next upkeep, choose a basic land type. This creature gains
//! landwalk of the chosen type until the end of that turn.`
//! GAP: "at the beginning of your next upkeep" — delayed upkeep trigger; no Effect variant
//! for scheduling a delayed "choose and grant landwalk". Full effect not expressible.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::effects::Effect;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Giant Slug");
    let slug = reg.interner_mut().intern("Slug");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(slug);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
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
                text: "{5}: At the beginning of your next upkeep, choose a basic land type. This creature gains landwalk of the chosen type until the end of that turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: schedule_landwalk,
            }),
    )
}

fn schedule_landwalk(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "at the beginning of your next upkeep, choose a basic land type, gain landwalk
    // until end of that turn" — no Effect for delayed-choice landwalk grant.
    Vec::new()
}
