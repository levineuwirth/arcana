//! Return to the Sewers — `{3}{U}` instant. "Target creature's owner
//! puts it on their choice of the top or bottom of their library. You
//! create a Mutagen token."
//!
//! The bounce-to-library is approximated as PutOnTopOfLibrary (the
//! "top or bottom" owner choice is not modeled). The Mutagen token is
//! an artifact with a complex activated sacrifice ability that has no
//! catalog representation.
//!
//! GAP: "owner's choice of top or bottom" not modeled (top used); the
//! Mutagen token's activated ability is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Return to the Sewers");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature's owner puts it on their choice of the top or bottom of their library. You create a Mutagen token.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: top/bottom owner choice not modeled (top used); Mutagen token ability not expressible.
    vec![Effect::PutOnTopOfLibrary { target: *id }]
}
