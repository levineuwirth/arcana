//! Sold Out — `{3}{B}` instant. "Exile target creature. If it was dealt
//! damage this turn, create a Clue token. (It's an artifact with
//! '{2}, Sacrifice this token: Draw a card.')"
//!
//! # GAP: "if it was dealt damage this turn" conditional check not
//! expressible via Effect::Conditional (no damage-this-turn predicate).
//! # GAP: Clue token creation (artifact token with activated ability)
//! not expressible — TokenDefinition has no `abilities` that are
//! activated abilities.
//! Emitting ExilePermanent only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sold Out");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target creature. If it was dealt damage this turn, create a Clue token.".into(),
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
    // GAP: conditional Clue token if dealt damage this turn not expressible
    vec![Effect::ExilePermanent { target: *id }]
}
