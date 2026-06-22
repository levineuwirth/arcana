//! Stoic Star-Captain — `{1}{W}` 2/3 Human Pilot (W).
//! "Each creature you control crews Vehicles and stations permanents as though
//! its power were 2 greater."
//! "Exhaust — {1}{W}: Seek a Spacecraft card."
//!
//! Seek and Exhaust are not modeled KeywordAbility variants. The crew/station
//! power-boost static has no Effect, so it is GAP'd. The activated ability pays
//! {1}{W}; its Seek effect and the Exhaust (once-per-game) restriction have no
//! engine surface, so the effect body is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stoic Star-Captain");
    let human = reg.interner_mut().intern("Human");
    let pilot = reg.interner_mut().intern("Pilot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(pilot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Each creature you control crews Vehicles and stations permanents as
    // though its power were 2 greater" — static crew/station modifier has no
    // engine effect.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: Exhaust (once-per-game) restriction and "Seek a Spacecraft
                // card" have no ActivationCost field / Effect variant.
                text: "Exhaust — {1}{W}: Seek a Spacecraft card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: seek_spacecraft,
            }),
    )
}

fn seek_spacecraft(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Seek a Spacecraft card" — no Effect::Seek variant.
    Vec::new()
}
