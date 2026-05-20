//! Waylay — `{2}{W}` instant. "Create three 2/2 white Knight
//! creature tokens. Exile them at the beginning of the next cleanup
//! step."
//!
//! The created token ids are not available to the resolver, so the
//! delayed exile of those tokens cannot be scheduled. We create the
//! three tokens; the cleanup-step exile is gapped.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Waylay");
    let _kn = reg.interner_mut().intern("Knight");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create three 2/2 white Knight creature tokens. Exile them at the beginning of the next cleanup step.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let kn = reg.interner().lookup("Knight").expect("Knight interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kn);
    let token = TokenDefinition {
        name: kn,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: created token ids are not available, so the delayed
    // cleanup-step exile of these tokens cannot be scheduled.
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
