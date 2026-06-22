//! Evra, Halcyon Witness — `{4}{W}{W}` 4/4 Legendary Creature — Avatar (W).
//! Lifelink.
//! `{4}: Exchange your life total with Evra's power.`
//!
//! Lifelink is a base keyword. The activated ability swaps the controller's
//! life total with this creature's power — there is no engine effect that
//! exchanges a life total with a creature's power, so it is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Evra, Halcyon Witness");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}: Exchange your life total with Evra's power.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exchange_life_with_power,
            }),
    )
}

fn exchange_life_with_power(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Exchange your life total with Evra's power" — no Effect variant
    // exchanges a player's life total with a creature's power.
    Vec::new()
}
