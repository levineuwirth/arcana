//! Balthor the Defiled — `{2}{B}{B}` 2/2 Legendary Zombie Dwarf.
//!
//! Minion creatures get +1/+1.  (static anthem — GAP)
//! `{B}{B}{B}, Exile Balthor: Each player returns all black and all red
//!  creature cards from their graveyard to the battlefield.`
//!
//! The mass-reanimation effect is GAP'd: there is no primitive that
//! returns ALL matching cards from each graveyard, and no helper to
//! enumerate graveyard card ids (ids_matching scans only the
//! battlefield; Reanimate returns a single creature). The activation
//! cost (`{B}{B}{B}` + exile this) is faithful; the body returns
//! `Vec::new()`.

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
    let name = reg.interner_mut().intern("Balthor the Defiled");
    let zombie = reg.interner_mut().intern("Zombie");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(dwarf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "Minion creatures get +1/+1" is a pure static anthem.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}{B}{B}, Exile Balthor: Each player returns all black and all red creature cards from their graveyard to the battlefield."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}{B}{B}").expect("valid cost"),
                    exile_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: mass_reanimate,
            }),
    )
}

fn mass_reanimate(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "each player returns ALL black and red creature cards from
    // their graveyard to the battlefield" — no mass-return primitive and
    // no graveyard-id enumeration helper. Reanimate returns one creature
    // only, which would be materially wrong vs "all".
    Vec::new()
}
