//! Servo Exhibition — Aether Revolt uncommon. `{1}{W}` sorcery,
//! "Create two 1/1 colorless Servo artifact creature tokens." The
//! seed's touchstone for CR 111 / CR 704.5d token creation: a pure
//! `Effect::CreateToken` ×2 with no rider triggers or conditional
//! logic, ideal for exercising the token path in isolation.
//!
//! # Rules references
//!
//! * CR 111.1 — A token is a marker representing any permanent not
//!   represented by a card. Tokens are permanents and obey all the
//!   ordinary permanent rules while on the battlefield.
//! * CR 111.4 — A token's information (name, P/T, etc.) is defined
//!   by the effect that creates it.
//! * CR 704.5d — "If a token is in a zone other than the
//!   battlefield, that token ceases to exist." Enforced by
//!   [`arcana_core::sba::apply_state_based_actions`]'s token-cease
//!   pass; a token that dies to combat damage / lethal damage / a
//!   destroy effect reaches the graveyard via the standard zone
//!   move and is then removed from the arena on the same SBA cycle.
//!
//! # Why this card
//!
//! Two tokens, identical spec, no triggers, no colors on the
//! tokens, no target requirements on the spell. Every mechanic
//! beyond `Effect::CreateToken` is off. Any engine gap this seed
//! finds is a gap in the token primitive, not in an adjacent
//! feature.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardRegistry, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Servo Exhibition");
    // "Servo" is both the token's name and its creature subtype. A
    // single interned SmallString serves both — `servo_resolve` looks
    // this up via the non-mut interner at resolve time.
    let _servo = reg.interner_mut().intern("Servo");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let ability = SpellAbilityDef {
        text: "Create two 1/1 colorless Servo artifact creature tokens.".into(),
        target_requirements: vec![],
        modal: None,
        effect: servo_resolve,
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(ability),
    )
}

fn servo_resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let servo = reg.interner().lookup("Servo")
        .expect("Servo interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(servo);
    let token = TokenDefinition {
        name: servo,
        colors: ColorSet::new(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
