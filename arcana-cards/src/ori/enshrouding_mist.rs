//! Enshrouding Mist — `{W}` Instant. "Target creature gets +1/+1 until
//! end of turn. Prevent all damage that would be dealt to it this turn.
//! If it's renowned, untap it."
//!
//! # Implementation note
//! The +1/+1 pump is expressible. Prevent-all-damage and the renown
//! conditional untap are not expressible with the current Effect
//! catalog.
//!
//! # GAP
//! Damage prevention not in Effect catalog; conditional untap
//! (if-renowned) not in Effect catalog.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Enshrouding Mist");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature gets +1/+1 until end of turn. Prevent all damage that would be dealt to it this turn. If it's renowned, untap it.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::Pump {
            target: *id,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        // GAP: prevent-all-damage not in Effect catalog
        // GAP: conditional untap (if renowned) not in Effect catalog
    ]
}
