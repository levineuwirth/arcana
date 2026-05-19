//! Vampires' Vengeance — `{2}{R}` instant, "Deals 2 damage to each non-Vampire
//! creature. Create a Blood token."
//!
//! # GAP: non-Vampire creature board-wipe damage — no Effect variant for dealing
//! damage to each creature matching a subtype exclusion filter; graveyard
//! enumeration API for non-Vampire filter not demonstrated.
//! # GAP: Blood token activated ability — TokenDefinition.abilities cannot express
//! "{1},{T},Discard a card, Sacrifice: Draw a card".

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vampires' Vengeance");
    let _blood = reg.interner_mut().intern("Blood");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Vampires' Vengeance deals 2 damage to each non-Vampire creature. Create a Blood token.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let blood = reg.interner().lookup("Blood").expect("Blood interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(blood);
    let blood_token = TokenDefinition {
        name: blood,
        colors: ColorSet::new(),
        types: TypeLine(TypeLine::ARTIFACT),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: non-Vampire board damage — cannot enumerate non-Vampire creatures with
    // the demonstrated API; omitting the damage effect.
    // GAP: Blood token activated ability not expressible in TokenDefinition.
    vec![
        Effect::CreateToken { controller: entry.controller, token: blood_token },
    ]
}
