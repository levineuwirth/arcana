//! Tolsimir Wolfblood — `{4}{G}{W}` 3/4 Legendary Elf Warrior.
//! "Other green creatures you control get +1/+1."
//! "Other white creatures you control get +1/+1."
//! "{T}: Create Voja, a legendary 2/2 green and white Wolf creature token."
//!
//! The two static anthems are continuous statics with no expressible
//! primitive — GAP'd. The {T} token-maker is wired (the token's
//! "legendary" supertype is not representable on a TokenDefinition).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tolsimir Wolfblood");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let _voja = reg.interner_mut().intern("Voja");
    let _wolf = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    // GAP: "Other green creatures you control get +1/+1" — static anthem.
    // GAP: "Other white creatures you control get +1/+1" — static anthem.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Create Voja, a legendary 2/2 green and white Wolf creature token.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: create_voja,
        }),
    )
}

fn create_voja(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let voja = reg.interner().lookup("Voja").unwrap_or_default();
    let wolf = reg.interner().lookup("Wolf").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: voja,
            colors: ColorSet::green() | ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
